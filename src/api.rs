use color_eyre::eyre::{Ok, Result};

use crate::models::PlaylistsResponse;

pub async fn get_playlists(access_token: &str) -> Result<PlaylistsResponse> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.spotify.com/v1/me/playlists")
        .bearer_auth(access_token)
        .send()
        .await?
        .error_for_status()?;

    let playlists = response.json().await?;
    Ok(playlists)
}
