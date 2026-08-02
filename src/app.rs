use ratatui::widgets::{ListState, TableState};

use crate::models::{Device, Playlist, Track};

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
    pub playlist_state: ListState,
    pub track_state: TableState,
    pub tracks: Vec<Track>,
    pub focus: Focus,
    pub devices: Vec<Device>,
    pub selected_device: Option<Device>,
}
