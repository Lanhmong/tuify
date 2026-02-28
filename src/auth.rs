mod config;
mod token;

pub use config::SpotifyConfig;
pub use token::{Token, format_expires_at, is_authorized, load_token, refresh, save_token};

use color_eyre::Result;
use oauth2::{
    AuthorizationCode, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, Scope, TokenResponse,
};
use rand::random;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::server;

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
            .authorize_url(|| CsrfToken::new(random::<u64>().to_string()))
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

pub fn start_auth_flow(config: &SpotifyConfig) -> Result<Token> {
    let auth_flow = AuthFlow::new(config)?;

    println!("Opening browser for authorization...");
    println!("Please authorize the application.");

    open::with_command(auth_flow.url(), "firefox").status()?;

    println!("Waiting for authorization...");
    let (code, received_state) = server::get_authorization_code()?;

    let token = auth_flow.exchange(&code, &received_state)?;

    Ok(token)
}
