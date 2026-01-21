use tokio::net::TcpListener;
use serde::Serialize;
use std::sync::{Arc, Mutex};
use std::net::SocketAddr;

use axum::{Router, Json, routing::get};
use tower_http::cors::CorsLayer;
use axum::http::{HeaderValue, Method};

use crate::chain::{Blockchain, COINBASE_MATURITY};

/* === Constants === */

const TARGET_BLOCK_TIME_SECONDS: u64 = 600;
const BLOCKS_PER_YEAR: u64 =
    (60 * 60 * 24 * 365) / TARGET_BLOCK_TIME_SECONDS;

/* === Responses === */

#[derive(Serialize)]
struct StatusResponse {
    height: usize,
    difficulty: u32,
    utxos: usize,
    total_mined: u64,
    circulating_supply: u64,
    inflation_rate: f64,
    blocks_until_halving: u64,
    halving_eta_seconds: u64,
}

/* === Helpers === */

fn total_mined(chain: &Blockchain) -> u64 {
    chain.blocks.iter().map(|b| {
        b.transactions.first()
            .map(|tx| tx.outputs.iter().map(|o| o.value).sum::<u64>())
            .unwrap_or(0)
    }).sum()
}

fn inflation_rate(total: u64, reward: u64) -> f64 {
    if total == 0 { return 0.0; }
    ((reward * BLOCKS_PER_YEAR) as f64 / total as f64) * 100.0
}

/* === Server === */

pub async fn start_api(chain: Arc<Mutex<Blockchain>>, port: u16) {
    let cors = CorsLayer::new()
        .allow_origin(
            "https://satoshi-nakamoto-itl.github.io"
                .parse::<HeaderValue>()
                .unwrap(),
        )
        .allow_methods([Method::GET]);

    let app = Router::new()
        .route("/status", get({
            let chain = Arc::clone(&chain);
            move || async move {
                let chain = chain.lock().unwrap();
                let height = chain.height();
                let total = total_mined(&chain);
                let reward = crate::reward::block_reward(height);

                Json(StatusResponse {
                    height: height as usize,
                    difficulty: chain.difficulty,
                    utxos: chain.utxos.len(),
                    total_mined: total,
                    circulating_supply: chain.utxos.values()
                        .filter(|u| {
                            u.height
                                .map(|h| height >= h + COINBASE_MATURITY)
                                .unwrap_or(false)
                        })
                        .map(|u| u.value)
                        .sum(),
                    inflation_rate: inflation_rate(total, reward),
                    blocks_until_halving: 210_000 - (height % 210_000),
                    halving_eta_seconds:
                        (210_000 - (height % 210_000)) * TARGET_BLOCK_TIME_SECONDS,
                })
            }
        }))
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("🌐 API running at http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
