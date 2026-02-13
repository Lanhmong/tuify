use sha2::{Digest, Sha256};

pub fn sha256_bytes(verifier: &str) -> [u8; 32] {
    Sha256::digest(verifier.as_bytes()).into()
}
