use std::collections::HashMap;
use tiny_http::{Response, Server};

pub fn run_server() {
    let server = tiny_http::Server::http("127.0.0.1:8080").expect("Failed to start server");

    let request = server.recv().expect("Failed to receive request");

    let url = request.url();
    let params = parse_params(&url);
    let code = params.get("code");
    let state = params.get("state");
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
