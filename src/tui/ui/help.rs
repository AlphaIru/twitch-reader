//! AlphaIru
//! Twitch Reader
//! 
//! tui/ui/help.rs
//!
//! This is the file that handles the rendering
//! the help menu for the tui.
//!     


use ratatui::{
    layout::Alignment,
    style::{Color, Style, Modifier},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Clear},
    Frame,
};
use super::utils::{centered_rect, create_help_line, create_header};

pub fn render_help_popup(f: &mut Frame) {
    let mut help_content = vec![
        Line::from(vec![
            Span::styled(" Twitch Reader TUI Help ", Style::default().bg(Color::Yellow).fg(Color::Black).add_modifier(Modifier::BOLD))
        ]).alignment(Alignment::Center),
        Line::from(""),
    ];

    help_content.push(create_header("[ Normal Mode ]"));
    help_content.push(create_help_line("i", "Enter Insert mode (chat)"));
    help_content.push(create_help_line(":", "Enter Command mode"));
    help_content.push(create_help_line("h", "Toggle help menu"));
    help_content.push(create_help_line("k/j", "Scroll logs (Up/Down)")); // 将来用
    help_content.push(Line::from(""));

    help_content.push(create_header("[ Insert Mode ]"));
    help_content.push(create_help_line("Enter", "Send message"));
    help_content.push(create_help_line("Esc", "Return to Normal mode"));
    help_content.push(Line::from(""));

    help_content.push(create_header("[ Command Mode ]"));
    help_content.push(create_help_line(":q", "Quit application"));
    help_content.push(create_help_line("Esc", "Cancel command"));


    let help_paragraph = Paragraph::new(help_content)
        .block(Block::default().title(" Help ").borders(Borders::ALL).border_style(Style::default().fg(Color::Yellow)));

    let area = centered_rect(70, 60, f.area());
    f.render_widget(Clear, area);
    f.render_widget(help_paragraph, area);
}
