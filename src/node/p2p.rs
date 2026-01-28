use std::collections::HashMap;
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH, Duration};

use crate::core::block::Block;
use crate::core::transaction::Transaction;
use crate::core::chain::Blockchain;
use crate::validation::validate_transaction;
use crate::node::message::{NetworkMessage, PROTOCOL_VERSION};

const MAX_MESSAGE_SIZE: usize = 1 * 1024 * 1024;
const MAX_ADDRS: usize = 32;

pub struct PeerNode {
    pub address: SocketAddr,
    pub last_seen: i64,
    pub stream: TcpStream,
}

pub struct P2PNetwork {
    peers: Arc<Mutex<HashMap<String, PeerNode>>>,
    listener_addr: SocketAddr,
    chain: Arc<Mutex<Blockchain>>,
}

impl P2PNetwork {
    pub fn new(chain: Arc<Mutex<Blockchain>>) -> Self {
        let listener = TcpListener::bind("0.0.0.0:0")
            .expect("P2P bind failed");
        listener.set_nonblocking(true).unwrap();

        let addr = listener.local_addr().unwrap();
        let peers = Arc::new(Mutex::new(HashMap::new()));

        let peers_accept = Arc::clone(&peers);
        let chain_accept = Arc::clone(&chain);

        // ───────── inbound peers ─────────
        thread::spawn(move || loop {
            match listener.accept() {
                Ok((stream, peer_addr)) => {
                    if peer_addr.ip().is_loopback() {
                        continue;
                    }

                    println!("🌐 Incoming peer {}", peer_addr);

                    stream
                        .set_read_timeout(Some(Duration::from_secs(30)))
                        .ok();

                    peers_accept.lock().unwrap().insert(
                        peer_addr.to_string(),
                        PeerNode {
                            address: peer_addr,
                            last_seen: now(),
                            stream: stream.try_clone().unwrap(),
                        },
                    );

                    let peers_inner = Arc::clone(&peers_accept);
                    let chain_inner = Arc::clone(&chain_accept);

                    thread::spawn(move || {
                        Self::handle_peer(stream, peers_inner, chain_inner);
                    });
                }
                Err(_) => thread::sleep(Duration::from_millis(100)),
            }
        });

        Self {
            peers,
            listener_addr: addr,
            chain,
        }
    }

    /// ───────── outbound connect ─────────
    pub fn connect(&self, addr: SocketAddr) {
        if addr.ip().is_loopback() {
            return;
        }

        if self.peers.lock().unwrap().contains_key(&addr.to_string()) {
            return;
        }

        match TcpStream::connect(addr) {
            Ok(mut stream) => {
                println!("🌍 Outbound peer connected: {}", addr);

                stream
                    .set_read_timeout(Some(Duration::from_secs(30)))
                    .ok();

                self.peers.lock().unwrap().insert(
                    addr.to_string(),
                    PeerNode {
                        address: addr,
                        last_seen: now(),
                        stream: stream.try_clone().unwrap(),
                    },
                );

                // ── HELLO ──
                let height = self.chain.lock().unwrap().height();
                let hello = NetworkMessage::Hello {
                    version: PROTOCOL_VERSION,
                    height,
                    agent: "bitcoin-v0.3.0-revelation".to_string(),
                };

                let _ = stream.write_all(
                    &bincode::serialize(&hello).unwrap()
                );

                // ── GETADDR ──
                let _ = stream.write_all(
                    &bincode::serialize(&NetworkMessage::GetAddr).unwrap()
                );

                let peers = Arc::clone(&self.peers);
                let chain = Arc::clone(&self.chain);

                thread::spawn(move || {
                    Self::handle_peer(stream, peers, chain);
                });
            }
            Err(e) => {
                println!("❌ Failed to connect {}: {}", addr, e);
            }
        }
    }

    fn handle_peer(
        mut stream: TcpStream,
        peers: Arc<Mutex<HashMap<String, PeerNode>>>,
        chain: Arc<Mutex<Blockchain>>,
    ) {
        loop {
            let mut buf = vec![0u8; MAX_MESSAGE_SIZE];
            let n = match stream.read(&mut buf) {
                Ok(0) | Err(_) => break,
                Ok(n) => n,
            };

            let msg: NetworkMessage = match bincode::deserialize(&buf[..n]) {
                Ok(m) => m,
                Err(_) => continue,
            };

            match msg {
                NetworkMessage::Hello { version, height, agent } => {
                    if version != PROTOCOL_VERSION {
                        println!("❌ Protocol mismatch from {}", stream.peer_addr().unwrap());
                        break;
                    }

                    println!(
                        "🤝 Handshake OK with {} [{}] height={}",
                        stream.peer_addr().unwrap(),
                        agent,
                        height
                    );

                    let local_height = chain.lock().unwrap().height();
                    if height > local_height {
                        let req = NetworkMessage::SyncRequest {
                            from_height: local_height,
                        };
                        let _ = stream.write_all(
                            &bincode::serialize(&req).unwrap()
                        );
                    }
                }

                NetworkMessage::GetAddr => {
                    let list: Vec<String> = peers
                        .lock()
                        .unwrap()
                        .keys()
                        .take(MAX_ADDRS)
                        .cloned()
                        .collect();

                    let _ = stream.write_all(
                        &bincode::serialize(
                            &NetworkMessage::Addr(list)
                        ).unwrap()
                    );
                }

                NetworkMessage::Addr(addrs) => {
                    for addr_str in addrs {
                        if let Ok(addr) = addr_str.parse::<SocketAddr>() {
                            if !addr.ip().is_loopback() {
                                let _ = TcpStream::connect(addr);
                            }
                        }
                    }
                }

                NetworkMessage::SyncRequest { from_height } => {
                    let c = chain.lock().unwrap();
                    for b in c.blocks.iter().skip(from_height as usize) {
                        let _ = stream.write_all(
                            &bincode::serialize(
                                &NetworkMessage::Block(b.clone())
                            ).unwrap()
                        );
                    }
                }

                NetworkMessage::Block(block) => {
                    chain.lock().unwrap()
                        .validate_and_add_block(block);
                }

                NetworkMessage::Transaction(tx) => {
                    let c = chain.lock().unwrap();
                    let _ = validate_transaction(&tx, &c.utxos, c.height());
                }

                NetworkMessage::Ping => {
                    let _ = stream.write_all(
                        &bincode::serialize(&NetworkMessage::Pong).unwrap()
                    );
                }

                NetworkMessage::Pong => {}
            }
        }

        if let Ok(addr) = stream.peer_addr() {
            println!("🔌 Disconnected {}", addr);
            peers.lock().unwrap().remove(&addr.to_string());
        }
    }

    pub fn request_sync(&self) {
        let height = self.chain.lock().unwrap().height();
        let msg = NetworkMessage::SyncRequest { from_height: height };
        let data = bincode::serialize(&msg).unwrap();

        for p in self.peers.lock().unwrap().values_mut() {
            let _ = p.stream.write_all(&data);
        }
    }

    pub fn broadcast_block(&self, block: &Block) {
        let data = bincode::serialize(
            &NetworkMessage::Block(block.clone())
        ).unwrap();

        for p in self.peers.lock().unwrap().values_mut() {
            let _ = p.stream.write_all(&data);
        }
    }

    pub fn broadcast_transaction(&self, tx: &Transaction) {
        let data = bincode::serialize(
            &NetworkMessage::Transaction(tx.clone())
        ).unwrap();

        for p in self.peers.lock().unwrap().values_mut() {
            let _ = p.stream.write_all(&data);
        }
    }

    pub fn peer_count(&self) -> usize {
        self.peers.lock().unwrap().len()
    }

    pub fn local_addr(&self) -> SocketAddr {
        self.listener_addr
    }
}

fn now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}
