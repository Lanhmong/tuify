use chrono::{DateTime, Local};
use color_eyre::Result;
use oauth2::{RefreshToken, TokenResponse};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::auth::SpotifyConfig;

#[derive(Serialize, Deserialize)]
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

pub fn is_authorized() -> bool {
    load_token()
        .map(|token| !token.is_expired())
        .unwrap_or(false)
}

pub fn save_token(token: &Token) -> Result<()> {
    let path = get_token_path();
    let content = serde_json::to_string_pretty(token)
        .map_err(|e| color_eyre::eyre::eyre!("Failed to serialize token: {}", e))?;
    fs::write(path, content)
        .map_err(|e| color_eyre::eyre::eyre!("Failed to write token file: {}", e))?;
    Ok(())
}

pub fn format_expires_at(expires_at: u64) -> String {
    let utc = DateTime::from_timestamp(expires_at as i64, 0).expect("invalid timestamp");
    let local = utc.with_timezone(&Local);
    local.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn refresh(config: &SpotifyConfig, refresh_token: &str) -> Result<Token> {
    let client = config.oauth_client();

    let token_result = client
        .exchange_refresh_token(&RefreshToken::new(refresh_token.to_string()))
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
