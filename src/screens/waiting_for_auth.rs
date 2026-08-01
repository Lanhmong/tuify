use crate::app::App;
use ratatui::{Frame, widgets::Paragraph};

use crate::app::Screen;

pub fn render(frame: &mut Frame) {
    let text = vec!["Waiting for authorization...".into()];
    frame.render_widget(Paragraph::new(text), frame.area());
}

pub fn on_token_received(app: &mut App, code: String) {
    app.access_token = Some(code);
    app.screen = Screen::Authenticated;
}
