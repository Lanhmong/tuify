use color_eyre::Result;
use oauth2::{
    basic::BasicClient, AuthUrl, AuthorizationCode, ClientId, CsrfToken, PkceCodeChallenge,
    PkceCodeVerifier, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Token {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: u64,
    pub scope: String,
}

impl Token {
    pub fn is_expired(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        now >= self.expires_at
    }
}

fn get_token_path() -> PathBuf {
    let config_dir = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("tuify");
    fs::create_dir_all(&config_dir).ok();
    config_dir.join("tokens.json")
}

pub fn load_token() -> Option<Token> {
    let path = get_token_path();
    let content = fs::read_to_string(path).ok()?;
    serde_json::from_str(&content).ok()
}

pub fn save_token(token: &Token) -> Result<()> {
    let path = get_token_path();
    let content = serde_json::to_string_pretty(token)
        .map_err(|e| color_eyre::eyre::eyre!("Failed to serialize token: {}", e))?;
    fs::write(path, content)
        .map_err(|e| color_eyre::eyre::eyre!("Failed to write token file: {}", e))?;
    Ok(())
}

pub struct AuthFlow {
    pkce_verifier: PkceCodeVerifier,
    csrf_token: CsrfToken,
    auth_url: String,
}

impl AuthFlow {
    pub fn new(config: &SpotifyConfig) -> Result<Self> {
        let client = config.oauth_client();

        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        let scopes: Vec<Scope> = config
            .scopes
            .iter()
            .map(|s| Scope::new(s.clone()))
            .collect();

        let (auth_url, csrf_token) = client
            .authorize_url(|| CsrfToken::new("dummy".to_string()))
            .add_scopes(scopes)
            .set_pkce_challenge(pkce_challenge)
            .url();

        Ok(Self {
            pkce_verifier,
            csrf_token,
            auth_url: auth_url.to_string(),
        })
    }

    pub fn url(&self) -> &str {
        &self.auth_url
    }

    pub fn state(&self) -> &str {
        self.csrf_token.secret()
    }

    pub fn exchange(self, code: &str, received_state: &str) -> Result<Token> {
        if received_state != self.csrf_token.secret() {
            return Err(color_eyre::eyre::eyre!(
                "State mismatch - possible CSRF attack"
            ));
        }

        let config = SpotifyConfig::from_env()?;
        let client = config.oauth_client();

        let token_result = client
            .exchange_code(AuthorizationCode::new(code.to_string()))
            .set_pkce_verifier(self.pkce_verifier)
            .request(oauth2::reqwest::http_client)?;

        let access_token = token_result.access_token().secret().to_string();
        let refresh_token = token_result.refresh_token().map(|t| t.secret().to_string());
        let expires_in = token_result
            .expires_in()
            .map(|e| e.as_secs())
            .unwrap_or(3600);

        let scope = config.scopes.join(" ");

        let expires_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + expires_in;

        Ok(Token {
            access_token,
            refresh_token,
            expires_at,
            scope,
        })
    }
}

pub fn refresh(config: &SpotifyConfig, refresh_token: &str) -> Result<Token> {
    let client = config.oauth_client();

    let token_result = client
        .exchange_refresh_token(&oauth2::RefreshToken::new(refresh_token.to_string()))
        .request(oauth2::reqwest::http_client)?;

    let access_token = token_result.access_token().secret().to_string();
    let new_refresh_token = token_result.refresh_token().map(|t| t.secret().to_string());
    let expires_in = token_result
        .expires_in()
        .map(|e| e.as_secs())
        .unwrap_or(3600);

    let scope = config.scopes.join(" ");

    let expires_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + expires_in;

    Ok(Token {
        access_token,
        refresh_token: new_refresh_token.or(Some(refresh_token.to_string())),
        expires_at,
        scope,
    })
}
