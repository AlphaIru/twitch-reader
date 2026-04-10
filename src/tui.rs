use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Modifier},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

fn parse_hex_color(hex: &str) -> Color {
    if hex.starts_with('#') && hex.len() == 7 {
        if let (Ok(r), Ok(g), Ok(b)) = (
            u8::from_str_radix(&hex[1..3], 16),
            u8::from_str_radix(&hex[3..5], 16),
            u8::from_str_radix(&hex[5..7], 16),
        ) {
            return Color::Rgb(r, g, b);
        }
    }
    Color::White
}

pub fn render(
    f: &mut Frame,
    logs: &[String],
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(f.area());

    let chat_height = chunks[1].height.saturating_sub(2) as usize;

    let display_start = logs.len().saturating_sub(chat_height);
    let visible_logs = &logs[display_start..];

    let status_info = format!(" Mode: Running ");
    let status = Paragraph::new(status_info)
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL).title(" Status "));
    f.render_widget(status, chunks[0]);

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

    let log_list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(" Chat Log (Esc to Quit) "));

    f.render_widget(log_list, chunks[1]);
}
