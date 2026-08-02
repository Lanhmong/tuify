use color_eyre::eyre::{Ok, Result};
use crossterm::event::{Event, KeyCode};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::{Block, List, ListItem},
};

use crate::{api::get_playlists_track, app::App};

pub fn render(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(33), Constraint::Percentage(67)])
        .split(frame.area());

    render_playlists(frame, app, chunks[0]);
    render_tracks(frame, app, chunks[1]);
}

fn render_playlists(frame: &mut Frame, app: &mut App, area: Rect) {
    let playlist_items: Vec<ListItem> = app
        .playlists
        .iter()
        .map(|p| ListItem::new(p.name.clone()))
        .collect();

    let list = List::new(playlist_items)
        .block(Block::bordered().title(" Playlists "))
        .highlight_symbol("> ")
        .highlight_style(Style::default().bg(ratatui::style::Color::Rgb(80, 80, 80)));

    frame.render_stateful_widget(list, area, &mut app.list_state);
}

fn render_tracks(frame: &mut Frame, app: &mut App, area: Rect) {
    let track_items: Vec<ListItem> = app
        .tracks
        .iter()
        .map(|t| ListItem::new(t.to_row()))
        .collect();

    let songs = List::new(track_items).block(Block::bordered().title(" Songs "));

    frame.render_widget(songs, area);
}

pub async fn update(app: &mut App, event: &Event) -> Result<()> {
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
            KeyCode::Enter => {
                if let (Some(token), Some(index)) = (&app.access_token, app.list_state.selected()) {
                    let playlist = &app.playlists[index];
                    let tracks = get_playlists_track(token, &playlist.id).await?;
                    app.tracks = tracks;
                }
            }
            _ => {}
        }
    }
    Ok(())
}
