use crate::models::Playlist;

pub enum Screen {
    Welcome,
    WaitingForAuth {
        rx: tokio::sync::mpsc::Receiver<String>,
        verifier: String,
    },
    Authenticated,
}

pub struct App {
    pub screen: Screen,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub playlists: Vec<Playlist>,
}
