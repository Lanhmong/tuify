use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph};
use ratatui::{DefaultTerminal, Frame};
mod server;
mod util;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
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
