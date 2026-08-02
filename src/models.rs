#[derive(serde::Deserialize)]
pub struct Playlist {
    pub name: String,
    pub id: String,
}

#[derive(serde::Deserialize)]
pub struct PlaylistsResponse {
    pub items: Vec<Playlist>,
}
