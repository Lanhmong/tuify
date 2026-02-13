use rand::{
    TryRng,
    rngs::{SysError, SysRng},
};

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
