mod app;
mod auth;
mod screens;

use color_eyre::eyre::Result;
use crossterm::event::{Event, EventStream, KeyCode, KeyModifiers};
use futures::StreamExt;
use ratatui::DefaultTerminal;

use crate::app::{App, Screen};
use crate::screens::welcome;
use crate::screens::{authenticated, waiting_for_auth};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let mut app_state = App {
        screen: Screen::Welcome,
        access_token: None,
        refresh_token: None,
        auth_rx: None,
    };
    let mut terminal = ratatui::init();
    let result = app(&mut terminal, &mut app_state).await;
    ratatui::restore();
    result
}

async fn app(terminal: &mut DefaultTerminal, app: &mut App) -> Result<()> {
    let mut events = EventStream::new();
    loop {
        terminal.draw(|f| match &app.screen {
            Screen::Welcome => welcome::render(f),
            Screen::WaitingForAuth => waiting_for_auth::render(f),
            Screen::Authenticated => authenticated::render(f),
        })?;

        tokio::select! {
            Some(Ok(event)) = events.next() => {
                if let Event::Key(key) = &event {
                    if key.code == KeyCode::Char('c')
                        && key.modifiers.contains(KeyModifiers::CONTROL)
                    {
                        break;
                    }
                }
                match &app.screen {
                    Screen::Welcome => welcome::update(app, &event)?,
                    Screen::WaitingForAuth => {}
                    Screen::Authenticated => {}
                }
            }
            code = async {
                match &mut app.auth_rx {
                    Some(rx) => rx.recv().await,
                    None => std::future::pending().await,
                }
            } => {
                if let Some(code) = code {
                    waiting_for_auth::on_token_received(app, code);
                }
            }
        }
    }
    Ok(())
}
