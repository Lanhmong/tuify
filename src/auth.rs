use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use color_eyre::eyre::{Ok, Result};
use rand::RngExt;
use sha2::{Digest, Sha256};
use tokio::sync::mpsc;
use url::{ParseError, Url};

const CLIENT_ID: &str = "bbc1f3f6cc4b4af5bb66cf2e6c83f1c8";

fn generate_random_string(length: usize) -> String {
    let possible = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::rng();
    let mut result = String::new();

    for _ in 0..length {
        let idx = rng.random_range(0..possible.len());
        let c = possible.as_bytes()[idx];
        result.push(c as char);
    }
    result
}

fn sha256(input: &str) -> Vec<u8> {
    let hash = Sha256::digest(input.as_bytes());
    hash.to_vec()
}

fn base64_encode(input: &[u8]) -> String {
    URL_SAFE_NO_PAD.encode(input)
}

fn generate_code_challenge() -> (String, String) {
    let verifier = generate_random_string(64);
    let hash = sha256(&verifier);
    let challenge = base64_encode(&hash);
    (verifier, challenge)
}

fn build_authorize_url(code_challenge: &str) -> Result<Url, ParseError> {
    let redirect_uri = "http://127.0.0.1:8080";
    let scope = "user-read-playback-state user-modify-playback-state playlist-read-private playlist-read-collaborative";
    Url::parse_with_params(
        "https://accounts.spotify.com/authorize",
        [
            ("response_type", "code"),
            ("client_id", CLIENT_ID),
            ("scope", scope),
            ("code_challenge_method", "S256"),
            ("code_challenge", code_challenge),
            ("redirect_uri", redirect_uri),
        ],
    )
}

pub fn authorize() -> Result<(String, Url)> {
    let (verifier, challenge) = generate_code_challenge();
    let url = build_authorize_url(&challenge)?;
    Ok((verifier, url))
}

pub fn run_server(tx: mpsc::Sender<String>) -> Result<()> {
    let server = tiny_http::Server::http("127.0.0.1:8080").unwrap();
    let request = server.recv().unwrap();

    // Build full URL from path, parse it, extract the "code" param
    let full_url = format!("http://127.0.0.1:8080{}", request.url());
    let parsed = Url::parse(&full_url).unwrap();
    let code = parsed
        .query_pairs()
        .find(|(key, _)| key == "code")
        .map(|(_, value)| value.to_string())
        .expect("no authorization code found in redirect");

    // Send the code back to the main thread
    tx.blocking_send(code)?;

    // Let the user know it worked
    request
        .respond(tiny_http::Response::from_string(
            "Logged in! You can close this tab.",
        ))
        .unwrap();
    Ok(())
}

#[derive(serde::Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub scope: String,
}

pub async fn exchange_code(code: &str, verifier: &str) -> Result<TokenResponse> {
    let client = reqwest::Client::new();

    let response = client
        .post("https://accounts.spotify.com/api/token")
        .form(&[
            ("grant_type", "authorization_code"),
            ("code", &code),
            ("redirect_uri", "http://127.0.0.1:8080"),
            ("client_id", CLIENT_ID),
            ("code_verifier", &verifier),
        ])
        .send()
        .await?;

    let tokens: TokenResponse = response.json().await?;
    Ok(tokens)
}
