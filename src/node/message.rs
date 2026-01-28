use serde::{Serialize, Deserialize};
use crate::core::block::Block;
use crate::core::transaction::Transaction;

pub const PROTOCOL_VERSION: u32 = 3;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkMessage {
    /// Initial handshake
    Hello {
        version: u32,
        height: u64,
        agent: String,
    },

    /// Ask peer for known addresses
    GetAddr,

    /// Peer address list
    Addr(Vec<String>),

    /// Request blocks from height
    SyncRequest {
        from_height: u64,
    },

    /// Block propagation
    Block(Block),

    /// Transaction gossip
    Transaction(Transaction),

    /// Keepalive
    Ping,
    Pong,
}
