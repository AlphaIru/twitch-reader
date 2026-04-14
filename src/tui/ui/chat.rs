//! AlphaIru
//! Twitch Reader
//! 
//! tui/ui/chat.rs
//!
//! This is the file that handles the rendering
//! the chat log of the tui.
//!     


use ratatui::{
    Frame,
    layout::Rect
};
use ratatui::widgets::{Block, Borders, List, ListItem};

use crate::tui::state::AppState;
use crate::tui::ui::utils::get_items;

pub fn render_chat_log(
    f: &mut Frame,
    chat_area: Rect,
    app_state: &AppState
) {
    let chat_height = chat_area.height.saturating_sub(2) as usize;
    let display_start = app_state.logs.len().saturating_sub(chat_height);
    let visible_logs = &app_state.logs[display_start..];

    let items: Vec<ListItem> = get_items(visible_logs);

    let log_list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(" Chat Log "));
    f.render_widget(log_list, chat_area);

}

