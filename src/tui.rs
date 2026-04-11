//! AlphaIru
//! Twitch Reader
//! 
//! tui.rs
//!
//! This is the file that handles the rendering
//! of the tui.
//!     

use tokio::sync::broadcast;

use crate::ChatPayload;

use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Modifier},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

fn parse_hex_color(hex: &str) -> Color {
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

    let status_info = (" TWITCH READER! ").to_string();
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

pub fn run_tui(
    broadcast_tx: broadcast::Sender<ChatPayload>
) -> Result<(), Box<dyn std::error::Error>> {
    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    crossterm::execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;
    let backend = ratatui::backend::CrosstermBackend::new(stdout);
    let mut terminal = ratatui::Terminal::new(backend)?;

    let mut logs: Vec<String> = Vec::new();
    let mut rx_for_tui_log = broadcast_tx.subscribe();

    let _ = broadcast_tx.send(ChatPayload {
        username: "[SYSTEM]".to_string(),
        user_id: "0".to_string(),
        msg: "Twitch Reader System started.".to_string(),
        color: "#FFFF66".to_string(),
        ..Default::default()
    });


    loop {
        while let Ok(payload) = rx_for_tui_log.try_recv() {
            let log_entry = format!(
                "{}|{}|{}|{}: {}",
                payload.color,
                payload.is_mod,
                payload.is_broadcaster,
                payload.username,
                payload.msg
            );
            
            logs.push(log_entry);

            if logs.len() > 50 {
                logs.remove(0);
            }
        }

        terminal.draw(|f| render(f, &logs))?;

        if crossterm::event::poll(std::time::Duration::from_millis(100))?
            && let crossterm::event::Event::Key(key) = crossterm::event::read()?
                && key.code == crossterm::event::KeyCode::Esc {
                    break; 
                }
    };

    crossterm::execute!(std::io::stdout(), crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(()) 
}
