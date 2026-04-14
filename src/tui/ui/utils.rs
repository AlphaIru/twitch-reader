//! AlphaIru
//! Twitch Reader
//! 
//! tui/ui/utils.rs
//!
//! This is the file that handles utility functions
//! for the tui.
//!     


use ratatui::{
    layout::{Constraint, Direction, Rect, Layout},
    style::{Color, Style, Modifier},
    text::{Line, Span},
    widgets::ListItem,
};


pub fn parse_hex_color(hex: &str) -> Color {
    if hex.starts_with('#') && hex.len() == 7
        && let (Ok(r), Ok(g), Ok(b)) = (
            u8::from_str_radix(&hex[1..3], 16),
            u8::from_str_radix(&hex[3..5], 16),
            u8::from_str_radix(&hex[5..7], 16),
        ) {
            return Color::Rgb(r, g, b);
    }
    Color::White
}


pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}


pub fn create_help_line<'a>(key: &'a str, desc: &'a str) -> Line<'a> {
    Line::from(vec![
        Span::raw("  "),
        Span::styled(format!("{:<7}", key), Style::default().fg(Color::Yellow)), 
        Span::raw(format!(": {}", desc)),
    ])
}


pub fn create_header<'a>(title: &'a str) -> Line<'a> {
    Line::from(vec![
        Span::raw(" "),
        Span::styled(title, Style::default().fg(Color::Cyan).add_modifier(Modifier::UNDERLINED)),
    ])
}


pub fn get_items<'a>(
    visible_logs: &'a[String]
) -> Vec<ListItem<'a>> {
    let items: Vec<ListItem> = visible_logs.iter().map(|log| {
        let parts: Vec<&str> = log.splitn(4, '|').collect();
        if parts.len() == 4 {
            let color_hex = parts[0];
            let is_mod = parts[1] == "true";
            let is_broadcaster = parts[2] == "true";

            if let Some((name, msg)) = parts[3].split_once(": ") {
                let user_color = parse_hex_color(color_hex);
                let mut spans = vec![];

                if is_broadcaster {
                    spans.push(Span::styled("[Broadcaster] ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)));
                }
                if is_mod {
                    spans.push(Span::styled("[Mod] ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)));
                }

                spans.push(Span::styled(format!("{}: ", name), Style::default().fg(user_color).add_modifier(Modifier::BOLD)));
                spans.push(Span::raw(msg));

                return ListItem::new(Line::from(spans));
            }
        }
        ListItem::new(Span::raw(log))
    }).collect();

    items
}


