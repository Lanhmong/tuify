use color_eyre::Result;
use ratatui::prelude::*;
use ratatui::widgets::Block;
use ratatui::{DefaultTerminal, Frame};
use std::sync::{Arc, Mutex};

mod auth;
mod server;

fn main() -> Result<()> {
    dotenv::dotenv().ok();
    color_eyre::install()?;

    let token = auth::load_token();

    let config = auth::SpotifyConfig::from_env()?;

    let token = if let Some(token) = token {
        if token.is_expired() {
            if let Some(refresh_token) = token.refresh_token {
                println!("Refreshing access token...");
                auth::refresh(&config, &refresh_token)?
            } else {
                println!("Token expired and no refresh token. Please re-authorize.");
                start_auth_flow(&config)?
            }
        } else {
            token
        }
    } else {
        start_auth_flow(&config)?
    };

    auth::save_token(&token)?;
    println!("Authenticated successfully!");

    ratatui::run(app)?;
    Ok(())
}

fn start_auth_flow(config: &auth::SpotifyConfig) -> Result<auth::Token> {
    let auth_flow = auth::AuthFlow::new(config)?;

    println!("Opening browser for authorization...");
    println!("Please authorize the application.");

    open::that(auth_flow.url())
        .map_err(|e| color_eyre::eyre::eyre!("Failed to open browser: {}", e))?;

    let callback = Arc::new(Mutex::new(server::AuthCallback::new()));

    println!("Waiting for authorization...");
    server::run_server(callback.clone())?;

    let cb = callback.lock().unwrap();

    if let Some(error) = &cb.error {
        return Err(color_eyre::eyre::eyre!("Authorization error: {}", error));
    }

    let code = cb
        .code
        .clone()
        .ok_or_else(|| color_eyre::eyre::eyre!("No code received"))?;
    let received_state = cb.state.clone().unwrap_or_default();

    drop(cb);

    let token = auth_flow.exchange(&code, &received_state)?;

    Ok(token)
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(render)?;
        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(3),
            Constraint::Percentage(50),
            Constraint::Min(1),
        ])
        .split(frame.area());

    let bottom_block = Block::bordered().title("Bottom");

    frame.render_widget(bottom_block, layout[1]);
}
