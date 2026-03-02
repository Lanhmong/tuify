use color_eyre::Result;
use crossterm::event::{Event, KeyCode, KeyModifiers, KeyEvent};
use ratatui::prelude::*;
use ratatui::widgets::{Block, List, ListItem, ListState, Paragraph};
use ratatui::{DefaultTerminal, Frame};

mod auth;
mod server;
mod spotify;

use crate::spotify::Playlist;

enum MoveDirection {
    Up,
    Down,
}

#[derive(Default)]
pub struct App {
    exit: bool,
    playlists: Option<Vec<Playlist>>,
    selected_playlist_index: usize,
    error: Option<String>,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        loop {
            terminal.draw(|f| self.render(f))?;
            if let Event::Key(key_event) = crossterm::event::read()? {
                if key_event.code == KeyCode::Char('c')
                    && key_event.modifiers.contains(KeyModifiers::CONTROL)
                {
                    return Ok(());
                }
                if let Some(ref playlists) = self.playlists
                    && !playlists.is_empty()
                {
                    let len = playlists.len();
                    self.handle_key_event(key_event, len);
                }
            }
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent, len: usize) {
        match key_event.code {
            KeyCode::Up | KeyCode::Char('k') => {
                self.move_selection(MoveDirection::Up, len);
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.move_selection(MoveDirection::Down, len);
            }
            _ => {}
        }
    }

    fn move_selection(&mut self, direction: MoveDirection, len: usize) {
        match direction {
            MoveDirection::Up => {
                if self.selected_playlist_index > 0 {
                    self.selected_playlist_index -= 1;
                }
            }
            MoveDirection::Down => {
                if self.selected_playlist_index < len - 1 {
                    self.selected_playlist_index += 1;
                }
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

        let block = Block::bordered().title("Playlists");

        let content = if let Some(ref error) = self.error {
            Text::from(format!("Error: {}", error))
        } else if let Some(ref playlists) = self.playlists {
            let items: Vec<ListItem> = playlists
                .iter()
                .map(|playlist| ListItem::new(playlist.name.as_str()))
                .collect();
            let mut list_state =
                ListState::default().with_selected(Some(self.selected_playlist_index));
            let list = List::new(items)
                .block(block)
                .highlight_symbol("> ")
                .highlight_style(
                    Style::new()
                        .bg(Color::DarkGray)
                        .add_modifier(Modifier::BOLD),
                );
            frame.render_stateful_widget(list, layout[1], &mut list_state);
            return;
        } else {
            Text::from("No playlists loaded")
        };

        let paragraph = Paragraph::new(content).block(block);
        frame.render_widget(paragraph, layout[1]);
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

    let (playlists, error) = match spotify::get_current_users_playlists() {
        Ok(playlists) => (Some(playlists), None),
        Err(e) => (None, Some(e.to_string())),
    };

    let mut app = App {
        playlists,
        error,
        exit: false,
        selected_playlist_index: 0,
    };

    ratatui::run(|terminal| app.run(terminal))?;
    Ok(())
}
