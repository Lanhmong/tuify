use color_eyre::Report;
use reqwest::blocking::Client;
use serde::Deserialize;

use crate::auth;

#[derive(Deserialize)]
pub struct PlaylistResponse {
    pub items: Vec<Playlist>,
}

#[derive(Deserialize)]
pub struct Playlist {
    pub id: String,
    pub name: String,
}

pub fn get_current_users_playlists() -> Result<Vec<Playlist>, Report> {
    let client = Client::new();
    let token = auth::load_token().ok_or_else(|| Report::msg("Not authenticated"))?;
    let response = client
        .get("https://api.spotify.com/v1/me/playlists")
        .header("Authorization", format!("Bearer {}", token.access_token))
        .send()?
        .text()?;

    let playlist_response: PlaylistResponse = serde_json::from_str(&response)?;
    Ok(playlist_response.items)
}
