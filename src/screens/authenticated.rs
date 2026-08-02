use ratatui::{
    Frame,
    widgets::{List, ListItem},
};

use crate::app::App;

pub fn render(frame: &mut Frame, app: &App) {
    let items: Vec<ListItem> = app
        .playlists
        .iter()
        .map(|p| ListItem::new(p.name.clone()))
        .collect();

    let list = List::new(items);

    frame.render_widget(list, frame.area());
}
