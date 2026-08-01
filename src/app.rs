pub enum Screen {
    Welcome,
    WaitingForAuth,
    Authenticated,
}

pub struct App {
    pub screen: Screen,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub auth_rx: Option<tokio::sync::mpsc::Receiver<String>>,
}
