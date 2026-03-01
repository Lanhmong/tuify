use color_eyre::Result;
use crossterm::event::{Event, KeyCode, KeyModifiers};
use ratatui::prelude::*;
use ratatui::widgets::Block;
use ratatui::{DefaultTerminal, Frame};

mod auth;
mod server;

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        loop {
            terminal.draw(|f| self.render(f))?;
            if let Event::Key(key_event) = crossterm::event::read()? {
                if (key_event.code == KeyCode::Char('c')
                    && key_event.modifiers.contains(KeyModifiers::CONTROL))
                {
                    break Ok(());
                }
                self.exit = !self.exit;
            }
        }
    }

    fn render(&mut self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(3),
                Constraint::Percentage(50),
                Constraint::Min(1),
            ])
            .split(frame.area());

        let bottom_block = Block::bordered().title(self.exit.to_string());

        frame.render_widget(bottom_block, layout[1]);
    }
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

    ratatui::run(|terminal| {
        let mut app = App::default();
        app.run(terminal)
    })?;
    Ok(())
}
