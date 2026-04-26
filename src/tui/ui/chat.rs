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
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};

use crate::tui::state::AppState;
use crate::tui::ui::utils::get_items;

pub fn render_chat_log(
    f: &mut Frame,
    chat_area: Rect,
    app_state: &AppState
) {
    let chat_height = chat_area.height.saturating_sub(2) as usize;
    let total_logs = app_state.logs.len();

    let max_scroll = total_logs.saturating_sub(chat_height);
    let effective_offset = (app_state.scroll_offset as usize).min(max_scroll);

    let display_end = total_logs.saturating_sub(effective_offset);
    let display_start = display_end.saturating_sub(chat_height);

    let visible_logs = if total_logs > 0 {
        &app_state.logs[display_start..display_end]
    } else {
        &[]
    };

    let items: Vec<ListItem> = get_items(
        visible_logs,
        display_start,
        app_state.selected_index,
    );

    let log_list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(" Chat Log "));
    f.render_widget(log_list, chat_area);

}

pub fn render_details(
    f: &mut Frame,
    details_area: Rect,
    app_state: &AppState
) {
    
    let text =  if let Some(selected) = app_state.logs.get(app_state.selected_index) {
        let chat_time = selected
            .sent_ts
            .and_then(chrono::DateTime::<chrono::Utc>::from_timestamp_millis)
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S%.3f UTC").to_string())
            .unwrap_or_else(|| "-".to_string());

        format!(
            "user_id: {}\nmessage_id: {}\nsent_ts: {}",
            selected.user_id,
            selected.message_id.as_deref().unwrap_or("-"),
            chat_time
        )
    } else {
        "No message selected.".to_string()
    };

    let details = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title(" Details "));
    f.render_widget(details, details_area);

}
