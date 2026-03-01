use color_eyre::Report;
use reqwest::blocking::Client;

use crate::auth;

pub fn get_current_users_playlist() -> Result<String, Report> {
    let client = Client::new();
    let token = auth::load_token().ok_or_else(|| Report::msg("Not authenticated"))?;
    let response = client
        .get("https://api.spotify.com/v1/me/playlists")
        .header("Authorization", format!("Bearer {}", token.access_token))
        .send()?
        .text()?;
    println!("{}", response);
    Ok(response)
}
