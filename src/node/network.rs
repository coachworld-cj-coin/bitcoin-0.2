use std::sync::{Arc, Mutex};
use std::net::SocketAddr;

use crate::core::block::Block;
use crate::core::transaction::Transaction;
use crate::core::chain::Blockchain;

// Internal P2P implementation
use crate::node::p2p::P2PNetwork as PrivateP2PNetwork;

/// Public-facing P2P wrapper
/// This layer intentionally exposes ONLY safe, non-consensus APIs
pub struct P2PNetwork {
    inner: PrivateP2PNetwork,
}

impl P2PNetwork {
    /// Create a new P2P network instance
    pub fn new(chain: Arc<Mutex<Blockchain>>) -> Self {
        let inner = PrivateP2PNetwork::new(chain);
        Self { inner }
    }

    /// Connect to a remote peer (IP:PORT)
    pub fn connect(&self, addr: SocketAddr) {
        self.inner.connect(addr);
    }

    /// Request chain synchronization from all peers
    pub fn request_sync(&self) {
        self.inner.request_sync();
    }

    /// Broadcast a newly mined block to peers
    pub fn broadcast_block(&self, block: &Block) {
        self.inner.broadcast_block(block);
    }

    /// Broadcast a validated transaction to peers
    /// NOTE: validation happens BEFORE calling this
    pub fn broadcast_transaction(&self, tx: &Transaction) {
        self.inner.broadcast_transaction(tx);
    }

    /// Number of currently connected peers
    pub fn peer_count(&self) -> usize {
        self.inner.peer_count()
    }

    /// Local socket address this node is listening on
    pub fn local_addr(&self) -> SocketAddr {
        self.inner.local_addr()
    }
}
