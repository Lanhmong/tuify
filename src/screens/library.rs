use color_eyre::eyre::{Ok, Result};
use crossterm::event::{Event, KeyCode};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, List, ListItem, Row, Table},
};

use crate::{
    api::get_playlists_track,
    app::{App, Focus},
};

pub fn render(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(33), Constraint::Percentage(67)])
        .split(frame.area());

    render_playlists(frame, app, chunks[0]);
    render_tracks(frame, app, chunks[1]);
}

fn render_playlists(frame: &mut Frame, app: &mut App, area: Rect) {
    let focused = matches!(app.focus, Focus::Playlists);
    let style = if focused {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };
    let playlist_items: Vec<ListItem> = app
        .playlists
        .iter()
        .map(|p| ListItem::new(p.name.clone()))
        .collect();

    let list = List::new(playlist_items)
        .block(Block::bordered().title(" Playlists ").border_style(style))
        .highlight_symbol("> ")
        .highlight_style(Style::default().bg(ratatui::style::Color::Rgb(80, 80, 80)));

    frame.render_stateful_widget(list, area, &mut app.list_state);
}

fn render_tracks(frame: &mut Frame, app: &mut App, area: Rect) {
    let focused = matches!(app.focus, Focus::Tracks);
    let style = if focused {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };
    let rows: Vec<Row> = app.tracks.iter().map(|t| Row::new(t.to_row())).collect();

    let table = Table::new(
        rows,
        [
            Constraint::Percentage(40),
            Constraint::Percentage(30),
            Constraint::Percentage(20),
            Constraint::Percentage(10),
        ],
    )
    .block(Block::bordered().title(" Songs ").border_style(style))
    .highlight_symbol("> ")
    .row_highlight_style(Style::default().bg(ratatui::style::Color::Rgb(80, 80, 80)));

    frame.render_stateful_widget(table, area, &mut app.track_state);
}

pub async fn update(app: &mut App, event: &Event) -> Result<()> {
    if let Event::Key(key) = event {
        match key.code {
            KeyCode::Char('j') | KeyCode::Down => match app.focus {
                Focus::Playlists => {
                    if app.playlists.is_empty() {
                        return Ok(());
                    }
                    let current = app.list_state.selected().unwrap_or(0);
                    if current + 1 < app.playlists.len() {
                        app.list_state.select(Some(current + 1));
                    }
                }
                Focus::Tracks => {
                    if app.tracks.is_empty() {
                        return Ok(());
                    }
                    let current = app.track_state.selected().unwrap_or(0);
                    if current + 1 < app.tracks.len() {
                        app.track_state.select(Some(current + 1));
                    }
                }
            },
            KeyCode::Char('k') | KeyCode::Up => match app.focus {
                Focus::Playlists => {
                    let current = app.list_state.selected().unwrap_or(0);
                    app.list_state.select(Some(current.saturating_sub(1)))
                }
                Focus::Tracks => {
                    let current = app.track_state.selected().unwrap_or(0);
                    app.track_state.select(Some(current.saturating_sub(1)))
                }
            },
            KeyCode::Enter => {
                if let (Some(token), Some(index)) = (&app.access_token, app.list_state.selected()) {
                    let playlist = &app.playlists[index];
                    let tracks = get_playlists_track(token, &playlist.id).await?;
                    app.tracks = tracks;
                    app.track_state.select(Some(0));
                    app.focus = Focus::Tracks;
                }
            }
            KeyCode::Tab => {
                app.focus = match app.focus {
                    Focus::Playlists => Focus::Tracks,
                    Focus::Tracks => Focus::Playlists,
                }
            }
            _ => {}
        }
    }
    Ok(())
}
