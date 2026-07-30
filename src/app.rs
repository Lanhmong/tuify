use std::sync::mpsc;

pub enum Screen {
    Welcome,
    WaitingForAuth {
        rx: mpsc::Receiver<String>,
        verifier: String,
    },
    Authenticated {
        access_token: String,
        refresh_token: String,
    },
}

pub struct App {
    pub screen: Screen,
}
