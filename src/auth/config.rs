use color_eyre::Result;
use oauth2::{basic::BasicClient, AuthUrl, ClientId, RedirectUrl, TokenUrl};

pub struct SpotifyConfig {
    pub client_id: String,
    pub redirect_uri: String,
    pub scopes: Vec<String>,
}

impl SpotifyConfig {
    pub fn from_env() -> Result<Self> {
        let client_id = std::env::var("SPOTIFY_CLIENT_ID")
            .map_err(|_| color_eyre::eyre::eyre!("SPOTIFY_CLIENT_ID env var not set"))?;

        Ok(Self {
            client_id,
            redirect_uri: "http://127.0.0.1:8080".to_string(),
            scopes: vec![
                "user-library-read".to_string(),
                "playlist-read-private".to_string(),
            ],
        })
    }

    pub fn oauth_client(&self) -> BasicClient {
        let client_id = ClientId::new(self.client_id.clone());
        let auth_url = AuthUrl::new("https://accounts.spotify.com/authorize".to_string())
            .expect("Invalid auth URL");
        let token_url = TokenUrl::new("https://accounts.spotify.com/api/token".to_string())
            .expect("Invalid token URL");

        BasicClient::new(client_id, None, auth_url, Some(token_url))
            .set_redirect_uri(RedirectUrl::new(self.redirect_uri.clone()).unwrap())
    }
}
