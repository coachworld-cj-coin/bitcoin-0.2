use bitcoin_v0_2_revelation::chain::Blockchain;
use bitcoin_v0_2_revelation::network::P2PNetwork;
use bitcoin_v0_2_revelation::api::start_api;
use bitcoin_v0_2_revelation::mempool::Mempool;
use bitcoin_v0_2_revelation::wallet::Wallet;

use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::{Duration, Instant};

use tokio::runtime::Runtime;

enum NodeMode {
    Syncing,
    Normal,
}

fn main() {
    println!("⛓ Bitcoin v0.2.1 — Revelation Edition (Consensus v2)");

    // ── Initialize blockchain ───────────────────────────────
    let mut local_chain = Blockchain::new();
    local_chain.initialize();

    let chain = Arc::new(Mutex::new(local_chain));
    let mempool = Arc::new(Mutex::new(Mempool::new()));

    // ── Create DEV WALLET ────────────────────────────────────
    let wallet = Wallet::new_dev();
    let miner_pubkey_hash = wallet.address();

    println!(
        "👛 Miner pubkey hash: {}",
        hex::encode(&miner_pubkey_hash)
    );

    // ── HTTP API ─────────────────────────────────────────────
    let api_chain = Arc::clone(&chain);

    thread::spawn(move || {
        let rt = Runtime::new().expect("Failed to create Tokio runtime");
        rt.block_on(start_api(api_chain, 8080));
    });

    println!("🌐 Explorer running at http://127.0.0.1:8080");

    // ── P2P NETWORK ──────────────────────────────────────────
    let p2p = P2PNetwork::new(Arc::clone(&chain));
    println!("🌐 P2P active at {}", p2p.local_addr());

    // ── NODE STATE ───────────────────────────────────────────
    let mut mode = NodeMode::Syncing;
    let mut last_height = chain.lock().unwrap().height();
    let mut last_change = Instant::now();

    let mut last_balance: u64 = 0;

    println!("🔄 Requesting sync from peers");
    p2p.request_sync();

    // ── MAIN LOOP ────────────────────────────────────────────
    loop {
        match mode {
            NodeMode::Syncing => {
                let height = chain.lock().unwrap().height();

                if height != last_height {
                    last_height = height;
                    last_change = Instant::now();
                }

                if last_change.elapsed() > Duration::from_secs(3) && height > 0 {
                    println!("✅ Sync complete at height {}", height);
                    mode = NodeMode::Normal;
                }

                sleep(Duration::from_millis(300));
            }

            NodeMode::Normal => {
                // STEP 1: select mempool transactions
                let mempool_txs = {
                    mempool.lock().unwrap().sorted_for_mining()
                };

                // STEP 2: mine block
                let latest_block = {
                    let mut chain = chain.lock().unwrap();
                    chain.mine_block(miner_pubkey_hash.clone(), mempool_txs);
                    chain.blocks.last().cloned()
                };

                // STEP 3: broadcast + cleanup
                if let Some(block) = latest_block {
                    p2p.broadcast_block(&block);

                    mempool
                        .lock()
                        .unwrap()
                        .remove_confirmed(&block.transactions);

                    let chain_guard = chain.lock().unwrap();
                    let balance = wallet.balance(&chain_guard.utxos);
                    let height = chain_guard.height();

                    if balance != last_balance {
                        println!(
                            "💰 Wallet balance: {} (height {})",
                            balance, height
                        );
                        last_balance = balance;
                    }
                }

                sleep(Duration::from_millis(100));
            }
        }
    }
}
