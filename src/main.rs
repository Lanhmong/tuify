mod app;
mod auth;
mod screens;

use color_eyre::eyre::Result;
use crossterm::event::{Event, KeyCode};
use ratatui::DefaultTerminal;

use crate::app::{App, Screen};
use crate::screens::welcome::{self, handle_enter};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let mut app_state = App {
        screen: Screen::Welcome,
    };
    ratatui::run(|terminal| app(terminal, &mut app_state))?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|f| match &app.screen {
            Screen::Welcome => welcome::render(f),
            Screen::WaitingForAuth { .. } => welcome::render(f), // placeholder
            Screen::Authenticated { .. } => welcome::render(f),  // placeholder
        })?;
        match &app.screen {
            Screen::Welcome => match crossterm::event::read()? {
                Event::Key(key_event) => match key_event.code {
                    KeyCode::Enter => app.screen = handle_enter()?,
                    KeyCode::Char('q') => break Ok(()),
                    _ => {}
                },
                _ => {}
            },
            Screen::WaitingForAuth { rx, verifier } => {}
            Screen::Authenticated {
                access_token,
                refresh_token,
            } => {}
        }
    }
}
