//! AlphaIru
//! Twitch Reader
//! 
//! tui/ui/mod.rs
//!
//! This is the file that handles the rendering
//! the main layout of the tui.
//!     



use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame
};
use crate::tui::state::AppState;

mod chat;
mod input;
mod help;
mod utils;

pub fn render(f: &mut Frame, app_state: &AppState) -> Rect {
    let chunks = if app_state.show_details {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0),
                Constraint::Length(5),
                Constraint::Length(3),
            ])
            .split(f.area())
    } else {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(f.area())
    };

    chat::render_chat_log(f, chunks[0], app_state);

    let input_rect = if app_state.show_details {
        chat::render_details(f, chunks[1], app_state);
        input::render_input_area(f, chunks[2], app_state);
        chunks[2]
    }
    else {
        input::render_input_area(f, chunks[1], app_state);
        chunks[1]
    };

    if app_state.show_help {
        help::render_help_popup(f);
    }

    input_rect
}
