use ratatui::{Frame, widgets::Paragraph};

pub fn render(frame: &mut Frame) {
    let text = vec!["Authenticated!".into()];
    frame.render_widget(Paragraph::new(text), frame.area());
}
