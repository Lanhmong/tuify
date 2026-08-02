use color_eyre::eyre::{Ok, Result, bail};
use crossterm::event::{Event, KeyCode};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, List, ListItem, ListState, Paragraph, Row, Table, TableState},
};

use crate::{
    api::{get_devices, get_playlists_track, play_track},
    app::{App, Focus},
    models::Device,
};

pub fn render(frame: &mut Frame, app: &mut App) {
    // 1. vertical: library (top) + status bar (bottom)
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(frame.area());

    // 2. horizontal split happens INSIDE chunks[0]
    let library = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(33), Constraint::Percentage(67)])
        .split(chunks[0]);

    render_playlists(frame, app, library[0]);
    render_tracks(frame, app, library[1]);
    render_status(frame, app, chunks[1]);
}

fn render_status(frame: &mut Frame, app: &mut App, area: Rect) {
    let text = match &app.selected_device {
        Some(device) => device.name.clone(),
        None => "press d to select a device".to_string(),
    };
    let status = Paragraph::new(text).block(Block::bordered().title(" Device "));
    frame.render_widget(status, area);
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

    frame.render_stateful_widget(list, area, &mut app.playlist_state);
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

fn move_playlist_selection(state: &mut ListState, len: usize, down: bool) {
    let current = state.selected().unwrap_or(0);
    let next = if down {
        (current + 1).min(len.saturating_sub(1))
    } else {
        current.saturating_sub(1)
    };
    state.select(Some(next));
}

fn move_track_selection(state: &mut TableState, len: usize, down: bool) {
    let current = state.selected().unwrap_or(0);
    let next = if down {
        (current + 1).min(len.saturating_sub(1))
    } else {
        current.saturating_sub(1)
    };
    state.select(Some(next));
}

fn advance_device_selection(devices: &[Device], current: Option<&Device>) -> Option<Device> {
    if devices.is_empty() {
        return None;
    }
    let current_idx = match current {
        Some(selected) => devices.iter().position(|d| d.id == selected.id),
        None => None,
    };
    let next = match current_idx {
        None => 0,
        Some(i) => (i + 1) % devices.len(),
    };
    Some(devices[next].clone())
}

pub async fn update(app: &mut App, event: &Event) -> Result<()> {
    if let Event::Key(key) = event {
        match key.code {
            KeyCode::Char('d') => {
                if let Some(token) = &app.access_token {
                    let devices = get_devices(token).await?;
                    app.selected_device =
                        advance_device_selection(&devices, app.selected_device.as_ref());
                    app.devices = devices;
                }
            }
            KeyCode::Char('j') | KeyCode::Down => match app.focus {
                Focus::Playlists => {
                    move_playlist_selection(&mut app.playlist_state, app.playlists.len(), true);
                }
                Focus::Tracks => {
                    move_track_selection(&mut app.track_state, app.tracks.len(), true);
                }
            },
            KeyCode::Char('k') | KeyCode::Up => match app.focus {
                Focus::Playlists => {
                    move_playlist_selection(&mut app.playlist_state, app.playlists.len(), false);
                }
                Focus::Tracks => {
                    move_track_selection(&mut app.track_state, app.tracks.len(), false);
                }
            },
            KeyCode::Enter => match app.focus {
                Focus::Playlists => {
                    if let (Some(token), Some(index)) =
                        (&app.access_token, app.playlist_state.selected())
                    {
                        let playlist = &app.playlists[index];
                        let tracks = get_playlists_track(token, &playlist.id).await?;
                        app.tracks = tracks;
                        app.track_state.select(Some(0));
                        app.focus = Focus::Tracks;
                    }
                }
                Focus::Tracks => {
                    if let (Some(token), Some(index)) =
                        (&app.access_token, app.track_state.selected())
                    {
                        let uri = app.tracks[index].uri.clone();
                        let device = match &app.selected_device {
                            Some(device) => Some(device.clone()),
                            None => {
                                let devices = get_devices(token).await?;
                                devices
                                    .iter()
                                    .find(|d| d.is_active)
                                    .or(devices.first())
                                    .cloned()
                            }
                        };
                        if let Some(device) = device {
                            play_track(token, &device.id, &uri).await?;
                        } else {
                            bail!("No Spotify device found - is the Spotify app open?")
                        }
                    }
                }
            },
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
