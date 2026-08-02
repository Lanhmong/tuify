use color_eyre::eyre::{Ok, Result};

use crate::models::{Playlist, Track};

#[derive(serde::Deserialize)]
struct PlaylistsResponse {
    items: Vec<Playlist>,
}

pub async fn get_playlists(access_token: &str) -> Result<Vec<Playlist>> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.spotify.com/v1/me/playlists")
        .bearer_auth(access_token)
        .send()
        .await?
        .error_for_status()?;

    let body: PlaylistsResponse = response.json().await?;
    Ok(body.items)
}

#[derive(serde::Deserialize)]
struct PlaylistTrackItem {
    item: Track,
}

#[derive(serde::Deserialize)]
struct PlaylistTrackResponse {
    items: Vec<PlaylistTrackItem>,
}

pub async fn get_playlists_track(access_token: &str, playlist_id: &str) -> Result<Vec<Track>> {
    let client = reqwest::Client::new();
    let response = client
        .get(format!(
            "https://api.spotify.com/v1/playlists/{playlist_id}/items"
        ))
        .bearer_auth(access_token)
        .send()
        .await?
        .error_for_status()?;

    let body: PlaylistTrackResponse = response.json().await?;
    Ok(body.items.into_iter().map(|i| i.item).collect())
}
