//! AlphaIru
//! Twitch Reader
//! 
//! tui/ui/mod.rs
//!
//! This is the file that handles the rendering
//! the main layout of the tui.
//!     



use ratatui::{layout::{Constraint, Direction, Layout}, Frame};
use crate::tui::state::AppState;

mod chat;
mod input;
mod help;
mod utils;

pub fn render(f: &mut Frame, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.area());

    chat::render_chat_log(f, chunks[0], app_state);
    input::render_input_area(f, chunks[1], app_state);

    if app_state.show_help {
        help::render_help_popup(f);
    }
}
