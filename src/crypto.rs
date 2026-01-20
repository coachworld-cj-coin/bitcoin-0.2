use sha2::{Sha256, Digest};
use ed25519_dalek::{PublicKey, Signature, Verifier};

/// Basic SHA256 helper
pub fn sha256(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

/// Verify an ed25519 signature
pub fn verify_signature(
    pubkey: &[u8],
    msg: &[u8],
    sig: &[u8],
) -> bool {
    let pk = match PublicKey::from_bytes(pubkey) {
        Ok(k) => k,
        Err(_) => return false,
    };

    let sig = match Signature::from_bytes(sig) {
        Ok(s) => s,
        Err(_) => return false,
    };

    pk.verify(msg, &sig).is_ok()
}
