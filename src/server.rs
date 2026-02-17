use color_eyre::{Result, eyre::WrapErr};
use std::{collections::HashMap, io::Error};
use tiny_http::{Response, Server};

pub fn run_server() -> Result<String> {
    let server = Server::http("127.0.0.1:8080")
        .map_err(|e| color_eyre::eyre::eyre!("Failed to start server: {}", e))?;

    let request = server
        .recv()
        .wrap_err("Failed to receive callback request")?;

    let params = parse_params(request.url());
    let code = params
        .get("code")
        .ok_or_else(|| color_eyre::eyre::eyre!("No code in callback"))
        .wrap_err("Invalid callback URL")?
        .to_string();

    request
        .respond(Response::from_string("Success!"))
        .wrap_err("Failed to send response to browser")?;

    Ok(code)
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
