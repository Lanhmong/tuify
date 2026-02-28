use color_eyre::Result;
use std::collections::HashMap;
use tiny_http::{Response, Server};

pub fn get_authorization_code() -> Result<(String, String)> {
    let server = Server::http("127.0.0.1:8080")
        .map_err(|e| color_eyre::eyre::eyre!("Failed to bind to port 8080: {}", e))?;

    println!("Server listening on http://127.0.0.1:8080");

    for request in server.incoming_requests() {
        let url = request.url();
        let params = parse_params(url);

        if let Some(error) = params.get("error") {
            let error = error.to_string();
            let response =
                Response::from_string("Authorization failed. You can close this window.");
            request.respond(response).ok();
            return Err(color_eyre::eyre::eyre!("Authorization error: {}", error));
        }

        if let (Some(code), Some(state)) = (params.get("code"), params.get("state")) {
            let code = code.to_string();
            let state = state.to_string();

            let response = Response::from_string(
                r#"/\_/\
( o.o )
 > ^ <

Authorization successful! You can close this window and return to the app."#,
            );
            request.respond(response).ok();
            return Ok((code, state));
        }

        let response = Response::from_string("Waiting for authorization...");
        request.respond(response).ok();
    }

    Err(color_eyre::eyre::eyre!("No authorization code received"))
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
