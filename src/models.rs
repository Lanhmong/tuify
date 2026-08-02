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
}

impl Track {
    pub fn to_row(&self) -> String {
        let artists = self
            .artists
            .iter()
            .map(|a| a.name.clone())
            .collect::<Vec<_>>()
            .join(", ");
        format!(
            "{} — {}  ·  {}  ·  {}:{:02}",
            self.name,
            artists,
            self.album.name,
            self.duration_ms / 60_000,
            (self.duration_ms % 60_000) / 1_000
        )
    }
}
