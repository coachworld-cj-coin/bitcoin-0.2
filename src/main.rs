use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::{Duration, Instant};
use std::env;
use std::net::SocketAddr;

use tokio::runtime::Runtime;

// Import core components
use bitcoin_v0_2_revelation::core::chain::Blockchain;
use bitcoin_v0_2_revelation::node::network::P2PNetwork;
use bitcoin_v0_2_revelation::node::mempool::Mempool;
use bitcoin_v0_2_revelation::interface::api::start_api;

enum NodeMode {
    Syncing,
    Normal,
}

/// Parse: --connect IP:PORT
fn parse_connect_arg(args: &[String]) -> Option<SocketAddr> {
    args.iter()
        .position(|a| a == "--connect")
        .and_then(|i| args.get(i + 1))
        .and_then(|s| s.parse().ok())
}

fn main() {
    println!("🌱 Bitcoin v0.3.2 — Revelation Edition");
    println!("🌍 SEED NODE MODE (no wallet, no mining)");

    // ───────── Blockchain ─────────
    let mut local_chain = Blockchain::new();
    local_chain.initialize();

    let chain = Arc::new(Mutex::new(local_chain));
    let mempool = Arc::new(Mutex::new(Mempool::new()));

    // ───────── Optional API (LOCAL ONLY) ─────────
    if env::var("ENABLE_API").is_ok() {
        let api_chain = Arc::clone(&chain);
        thread::spawn(move || {
            let rt = Runtime::new().expect("Tokio runtime failed");
            rt.block_on(start_api(api_chain, 8080));
        });

        println!("🌐 Local API enabled on 127.0.0.1:8080");
    }

    // ───────── P2P Network ─────────
    let p2p = P2PNetwork::bind("0.0.0.0:8333", Arc::clone(&chain));
    println!("🔗 P2P seed listening on 0.0.0.0:8333");

    let args: Vec<String> = env::args().collect();
    if let Some(addr) = parse_connect_arg(&args) {
        println!("🌍 Connecting to peer {}", addr);
        p2p.connect(addr);
    }

    println!("🔄 Requesting sync from peers");
    p2p.request_sync();

    // ───────── Main Loop ─────────
    let mut mode = NodeMode::Syncing;
    let mut last_height = chain.lock().unwrap().height();
    let mut last_change = Instant::now();

    loop {
        match mode {
            NodeMode::Syncing => {
                let height = chain.lock().unwrap().height();

                if height != last_height {
                    last_height = height;
                    last_change = Instant::now();
                }

                if last_change.elapsed() > Duration::from_secs(5) && height > 0 {
                    println!("✅ Seed synced at height {}", height);
                    mode = NodeMode::Normal;
                }

                sleep(Duration::from_millis(500));
            }

            NodeMode::Normal => {
                // Seed node does NOT mine.
                // It only relays blocks & transactions via P2P.
                sleep(Duration::from_secs(1));
            }
        }
    }
}
