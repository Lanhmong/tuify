use crate::{app::App, auth::run_server};
use color_eyre::{Result, eyre::Ok};
use crossterm::event::{Event, KeyCode};
use ratatui::{Frame, widgets::Paragraph};
use tokio::sync::mpsc;

use crate::{app::Screen, auth::authorize};

pub fn render(frame: &mut Frame) {
    let text = vec![
        "Welcome to Tuify!".into(),
        "Press Enter to authorize.".into(),
    ];
    frame.render_widget(Paragraph::new(text), frame.area());
}

pub fn update(app: &mut App, event: &Event) -> Result<()> {
    if let Event::Key(key) = event {
        match key.code {
            KeyCode::Enter => handle_enter(app)?,
            _ => {}
        }
    }
    Ok(())
}

pub fn handle_enter(app: &mut App) -> Result<()> {
    let (_verifier, url) = authorize()?;
    let (tx, rx) = mpsc::channel(1);
    let _ = tokio::task::spawn_blocking(move || run_server(tx));
    webbrowser::open(url.as_str())?;
    app.auth_rx = Some(rx);
    app.screen = Screen::WaitingForAuth;
    Ok(())
}
