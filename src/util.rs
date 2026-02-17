use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use rand::{
    TryRng,
    rngs::{SysError, SysRng},
};
use sha2::{Digest, Sha256};

pub fn get_random_string() -> Result<String, SysError> {
    const POSSIBLE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = SysRng;
    let mut out = String::with_capacity(64);
    let mut b = [0u8; 1];

    for _ in 0..64 {
        rng.try_fill_bytes(&mut b)?;
        let idx = (b[0] as usize) % POSSIBLE.len();
        out.push(POSSIBLE[idx] as char);
    }

    Ok(out)
}

pub fn sha256_bytes(verifier: &str) -> [u8; 32] {
    Sha256::digest(verifier.as_bytes()).into()
}

pub fn base64_encode(input: &[u8]) -> String {
    URL_SAFE_NO_PAD.encode(input)
}

pub fn generate_challenge(verifier: &str) -> String {
    let hash = sha256_bytes(verifier);
    base64_encode(&hash)
}

pub fn generate_pair() -> Result<(String, String), SysError> {
    let verifier = get_random_string()?;
    let challenge = generate_challenge(&verifier);
    Ok((verifier, challenge))
}
