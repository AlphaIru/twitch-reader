//! AlphaIru
//! Twitch Reader
//! 
//! tui/mod.rs 
//!
//! This is the file that handles the connection
//! with the main tui. 
//!     

use std::time::Duration;
use crossterm::event::{self, Event};

use tokio::sync::{broadcast, mpsc, oneshot};

use crate::ChatPayload;

mod ui;
mod events;
pub mod state;

pub use state::AppState;


pub async fn run_tui (
    broadcast_tx: broadcast::Sender<ChatPayload>,
    narrowcast_tx: mpsc::Sender<String>,
    mut config_rx: oneshot::Receiver<(String, String)>,
) -> Result<(), Box<dyn std::error::Error>> {
    
    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    crossterm::execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;
    let backend = ratatui::backend::CrosstermBackend::new(stdout);
    let mut terminal = ratatui::Terminal::new(backend)?;

    let _ = broadcast_tx.send(ChatPayload {
        username: "[SYSTEM]".to_string(),
        user_id: "0".to_string(),
        msg: "Twitch Reader System started.".to_string(),
        color: "#FFFF66".to_string(),
        ..Default::default()
    });


    let mut state = AppState::new();
    let mut broadcast_rx = broadcast_tx.subscribe();
    let mut config_loaded = false;

    loop 
    {
        if !config_loaded {
            match config_rx.try_recv() {
                Ok((name, color)) => {
                    state.my_name = name;
                    state.my_color = color;
                    config_loaded = true;
                }
                Err(tokio::sync::oneshot::error::TryRecvError::Empty) => {}
                Err(tokio::sync::oneshot::error::TryRecvError::Closed) => {
                    config_loaded = true;
                }
            }
        }

        while let Ok(payload) = broadcast_rx.try_recv() {
            // state.push_log(format!("{}: {}", payload.username, payload.msg));

            let log_entry = format!(
                "{}|{}|{}|{}: {}",
                payload.color,
                payload.is_mod,
                payload.is_broadcaster,
                payload.username,
                payload.msg
            );
            state.push_log(log_entry);
        }

        terminal.draw(|f| ui::render(f, &state))?;

        if !event::poll(Duration::from_millis(50))? {
            continue;
        }
        let Event::Key(key) = event::read()? else { continue };
        if !events::handle_key(
            key,
            &mut state,
            narrowcast_tx.clone(),
        ).await
        {
            break;
        }
    }
    
    crossterm::execute!(std::io::stdout(), crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}

