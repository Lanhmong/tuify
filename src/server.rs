use color_eyre::Result;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tiny_http::{Response, Server};

pub struct AuthCallback {
    pub code: Option<String>,
    pub state: Option<String>,
    pub error: Option<String>,
    pub received: bool,
}

impl AuthCallback {
    pub fn new() -> Self {
        Self {
            code: None,
            state: None,
            error: None,
            received: false,
        }
    }
}

pub fn run_server(callback: Arc<Mutex<AuthCallback>>) -> Result<()> {
    let server = Server::http("127.0.0.1:8080")
        .map_err(|e| color_eyre::eyre::eyre!("Failed to bind to port 8080: {}", e))?;

    println!("Server listening on http://127.0.0.1:8080");

    for request in server.incoming_requests() {
        let url = request.url();
        let params = parse_params(url);

        let mut cb = callback.lock().unwrap();

        if let Some(error) = params.get("error") {
            cb.error = Some(error.to_string());
            cb.received = true;

            let response =
                Response::from_string("Authorization failed. You can close this window.");
            request.respond(response).ok();
            break;
        }

        if let Some(code) = params.get("code") {
            let state = params.get("state").map(|s| s.to_string());
            cb.code = Some(code.to_string());
            cb.state = state;
            cb.received = true;

            let response = Response::from_string(
                "Authorization successful! You can close this window and return to the app.",
            );
            request.respond(response).ok();
            break;
        }

        let response = Response::from_string("Waiting for authorization...");
        request.respond(response).ok();
    }

    Ok(())
}

fn parse_params(url: &str) -> HashMap<&str, &str> {
    let query = url.split('?').nth(1).unwrap_or("");
    query
        .split('&')
        .filter_map(|pair| {
            let mut parts = pair.split('=');
            Some((parts.next()?, parts.next().unwrap_or("")))
        })
        .collect()
}
