use ratatui::widgets::ListState;

use crate::models::{Playlist, Track};

pub enum Screen {
    Welcome,
    WaitingForAuth {
        rx: tokio::sync::mpsc::Receiver<String>,
        verifier: String,
    },
    Library,
}

pub struct App {
    pub screen: Screen,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub playlists: Vec<Playlist>,
    pub list_state: ListState,
    pub tracks: Vec<Track>,
}
