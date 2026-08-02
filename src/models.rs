#[derive(serde::Deserialize)]
pub struct Playlist {
    pub name: String,
    pub id: String,
}

#[derive(serde::Deserialize)]
pub struct Artist {
    pub name: String,
}

#[derive(serde::Deserialize)]
pub struct Album {
    pub name: String,
}

#[derive(serde::Deserialize)]
pub struct Track {
    pub name: String,
    pub duration_ms: u64,
    pub artists: Vec<Artist>,
    pub album: Album,
    pub uri: String,
}

impl Track {
    pub fn to_row(&self) -> Vec<String> {
        let artists = self
            .artists
            .iter()
            .map(|a| a.name.clone())
            .collect::<Vec<_>>()
            .join(", ");
        vec![
            self.name.clone(),
            artists,
            self.album.name.clone(),
            format!(
                "{}:{:02}",
                self.duration_ms / 60_000,
                (self.duration_ms % 60_000) / 1_000
            ),
        ]
    }
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Device {
    pub id: String,
    pub name: String,
    pub is_active: bool,
}
