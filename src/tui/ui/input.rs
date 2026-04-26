//! AlphaIru
//! Twitch Reader
//! 
//! tui/ui/input.rs
//!
//! This is the file that handles the rendering
//! the input area for the tui.
//!     


use ratatui::{
    layout::Rect,
    style::{Color, Style, Modifier},
    text::Span,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::tui::state::{AppState, InputMode};


pub fn render_input_area(
    f: &mut Frame,
    input_area: Rect,
    app_state: &AppState
) {

    let (mode_label, mode_color, display_text) = match app_state.mode {
        InputMode::Normal => (
            "NORMAL",
            Color::Blue,
            app_state.input_text.clone(),
        ),
        InputMode::Insert => (
            "INSERT",
            Color::Green,
            app_state.input_text.clone(),
        ),
        InputMode::Command => (
            "COMMAND",
            Color::Magenta,
            format!(":{}", app_state.input_text.clone()),
        ),
    };

    let input_widget = Paragraph::new(display_text.as_str())
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(mode_color))
            .title(
                Span::styled(
                    mode_label,
                    Style::default().fg(mode_color).add_modifier(Modifier::BOLD),
                )
            )
        );
    f.render_widget(input_widget, input_area);
}


