use std::sync::mpsc::Receiver;

use crate::server::CallbackResult;
use crate::util;

const CLIENT_ID: &str = "bbc1f3f6cc4b4af5bb66cf2e6c83f1c8";
const REDIRECT_URI: &str = "http://127.0.0.1:8080";
const SCOPES: &str =
    "user-read-private user-read-email user-read-playback-state user-modify-playback-state";

pub struct AuthState {
    pub receiver: Receiver<CallbackResult>,
    pub code_verifier: String,
}

pub fn start_login() -> Option<AuthState> {
    let (code_verifier, code_challenge) = util::generate_pair().ok()?;

    let auth_url = format!(
        "https://accounts.spotify.com/authorize?response_type=code&client_id={}&scope={}&code_challenge_method=S256&code_challenge={}&redirect_uri={}",
        CLIENT_ID,
        urlencoding::encode(SCOPES),
        code_challenge,
        urlencoding::encode(REDIRECT_URI),
    );

    let receiver = crate::server::start_callback_server(8080);

    if let Err(e) = open::that(&auth_url) {
        eprintln!("Failed to open browser: {}", e);
        return None;
    }

    Some(AuthState {
        receiver,
        code_verifier,
    })
}

pub fn check_callback(auth_state: &AuthState) -> Option<(String, String)> {
    match auth_state.receiver.try_recv() {
        Ok(result) => Some((result.code, auth_state.code_verifier.clone())),
        Err(std::sync::mpsc::TryRecvError::Empty) => None,
        Err(std::sync::mpsc::TryRecvError::Disconnected) => {
            eprintln!("Callback server disconnected");
            None
        }
    }
}
