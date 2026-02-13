use rand::rngs::SysError;

mod base64;
mod random_string;
mod sha256;

fn main() -> Result<(), SysError> {
    let code_verifier = random_string::get_random_string()?;
    let hashed = sha256::sha256_bytes(&code_verifier);
    let code_challenge = base64::base64_encode(&hashed);

    println!("code verifier: {code_verifier}");
    println!("code challenge: {code_challenge}");
    Ok(())
}
