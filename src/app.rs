use ratatui::widgets::{ListState, TableState};

use crate::models::{Playlist, Track};

pub enum Screen {
    Welcome,
    WaitingForAuth {
        rx: tokio::sync::mpsc::Receiver<String>,
        verifier: String,
    },
    Library,
}

pub enum Focus {
    Playlists,
    Tracks,
}

pub struct App {
    pub screen: Screen,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub playlists: Vec<Playlist>,
    pub list_state: ListState,
    pub track_state: TableState,
    pub tracks: Vec<Track>,
    pub focus: Focus,
}
