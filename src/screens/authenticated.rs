use color_eyre::eyre::{Ok, Result};
use crossterm::event::{Event, KeyCode};
use ratatui::{
    Frame,
    style::Style,
    widgets::{List, ListItem},
};

use crate::app::App;

pub fn render(frame: &mut Frame, app: &mut App) {
    let items: Vec<ListItem> = app
        .playlists
        .iter()
        .map(|p| ListItem::new(p.name.clone()))
        .collect();

    let list = List::new(items)
        .highlight_symbol("> ")
        .highlight_style(Style::default().bg(ratatui::style::Color::Rgb(80, 80, 80)));

    frame.render_stateful_widget(list, frame.area(), &mut app.list_state);
}

pub fn update(app: &mut App, event: &Event) -> Result<()> {
    if let Event::Key(key) = event {
        match key.code {
            KeyCode::Char('j') => {
                if app.playlists.is_empty() {
                    return Ok(());
                }
                let current = app.list_state.selected().unwrap_or(0);
                if current + 1 < app.playlists.len() {
                    app.list_state.select(Some(current + 1));
                }
            }
            KeyCode::Char('k') => {
                let current = app.list_state.selected().unwrap_or(0);
                app.list_state.select(Some(current.saturating_sub(1)))
            }
            _ => {}
        }
    }
    Ok(())
}
