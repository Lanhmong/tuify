use crate::{api::get_playlists, app::App, auth::exchange_code};
use color_eyre::Result;
use ratatui::{Frame, widgets::Paragraph};

use crate::app::Screen;

pub fn render(frame: &mut Frame) {
    let text = vec!["Waiting for authorization...".into()];
    frame.render_widget(Paragraph::new(text), frame.area());
}

pub async fn on_token_received(app: &mut App, code: &str) -> Result<()> {
    let verifier = match &app.screen {
        Screen::WaitingForAuth { verifier, .. } => verifier,
        _ => unreachable!(),
    };

    let tokens = exchange_code(code, verifier).await?;
    let playlists = get_playlists(&tokens.access_token).await?;
    app.access_token = Some(tokens.access_token);
    app.refresh_token = Some(tokens.refresh_token);
    app.playlists = playlists;
    app.playlist_state.select(Some(0));
    app.screen = Screen::Library;
    Ok(())
}
