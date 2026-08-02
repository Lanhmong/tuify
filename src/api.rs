use color_eyre::eyre::{Ok, Result};

use crate::models::{Device, Playlist, Track};

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

#[derive(serde::Deserialize)]
struct DevicesResponse {
    devices: Vec<Device>,
}

pub async fn get_devices(access_token: &str) -> Result<Vec<Device>> {
    let client = reqwest::Client::new();
    let response = client
        .get(format!("https://api.spotify.com/v1/me/player/devices"))
        .bearer_auth(access_token)
        .send()
        .await?
        .error_for_status()?;

    let body: DevicesResponse = response.json().await?;
    Ok(body.devices)
}

pub async fn play_track(access_token: &str, device_id: &str, track_uri: &str) -> Result<()> {
    let client = reqwest::Client::new();
    client
        .put(format!(
            "https://api.spotify.com/v1/me/player/play?device_id={device_id}"
        ))
        .bearer_auth(access_token)
        .json(&serde_json::json!({ "uris": [track_uri] }))
        .send()
        .await?
        .error_for_status()?;
    Ok(())
}
