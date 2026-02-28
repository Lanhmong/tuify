use color_eyre::Result;
use ratatui::prelude::*;
use ratatui::widgets::Block;
use ratatui::{DefaultTerminal, Frame};

mod auth;
mod server;

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
}

fn main() -> Result<()> {
    dotenv::dotenv().ok();
    color_eyre::install()?;

    let config = auth::SpotifyConfig::from_env()?;

    let token = if auth::is_authorized() {
        let token = auth::load_token().unwrap();
        let datetime = auth::format_expires_at(token.expires_at);
        println!("Token runs out at {}", datetime);
        if let Some(refresh_token) = token.refresh_token {
            println!("Refreshing access token...");
            auth::refresh(&config, &refresh_token)?
        } else {
            token
        }
    } else {
        auth::start_auth_flow(&config)?
    };

    auth::save_token(&token)?;

    ratatui::run(app)?;
    Ok(())
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
