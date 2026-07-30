use crate::auth;
use color_eyre::Result;
use color_eyre::eyre::Ok;
use ratatui::{Frame, widgets::Paragraph};
use std::sync::mpsc;

use crate::{app::Screen, auth::authorize};

pub fn render(frame: &mut Frame) {
    let text = vec![
        "Welcome to Tuify!".into(),
        "Press Enter to authorize.".into(),
    ];
    frame.render_widget(Paragraph::new(text), frame.area());
}

pub fn handle_enter() -> Result<Screen> {
    let (verifier, url) = authorize()?;
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        auth::run_server(tx);
    });
    webbrowser::open(url.as_str())?;
    Ok(Screen::WaitingForAuth { rx, verifier })
}
