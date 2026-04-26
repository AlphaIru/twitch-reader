//! AlphaIru
//! Twitch Reader
//! 
//! tui/mod.rs 
//!
//! This is the file that handles the connection
//! with the main tui. 
//!     

use std::{
    io::stdout,
    time::Duration,
};

use crossterm::{
    cursor::Show,
    execute,
    event::{self, Event},
    terminal::{
        enable_raw_mode,
        disable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};

use tokio::sync::{broadcast, mpsc};

use crate::twitch::types::{ChatPayload, Outgoing};

mod ui;
mod events;
pub mod state;

pub use state::AppState;
pub use crate::helix::user::BroadcasterProfile;

pub async fn run_tui (
    broadcast_tx: broadcast::Sender<ChatPayload>,
    narrowcast_tx: mpsc::Sender<Outgoing>,
    broadcaster_profile: BroadcasterProfile,
) -> Result<(), Box<dyn std::error::Error>> {
    
    enable_raw_mode()?;
    let mut tui_stdout = stdout();
    execute!(
        tui_stdout,
        Show,
        EnterAlternateScreen
    )?;
    let backend = CrosstermBackend::new(tui_stdout);
    let mut terminal = Terminal::new(backend)?;

    let _ = broadcast_tx.send(ChatPayload {
        username: "[SYSTEM]".to_string(),
        msg: "Twitch Reader System started.".to_string(),
        color: "#FFFF66".to_string(),
        ..Default::default()
    });


    let mut state = AppState::new();
    let mut broadcast_rx = broadcast_tx.subscribe();

    state.my_profile = broadcaster_profile;

    loop 
    {
        while let Ok(payload) = broadcast_rx.try_recv() {
            state.push_log(payload);
        }

        terminal.draw(|f| {
            let input_rect = ui::render(f, &state);

            match state.mode {
                state::InputMode::Insert => {
                    f.set_cursor_position((
                        input_rect.x + 1 + state.input_text.chars().count() as u16,
                        input_rect.y + 1
                    ));
                }
                state::InputMode::Command => {
                    f.set_cursor_position((
                        input_rect.x + 2 + state.input_text.chars().count() as u16,
                        input_rect.y + 1
                    ));
                }
                state::InputMode::Normal => {}
            }
        
        })?;

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
   
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        Show,
        LeaveAlternateScreen
    )?;

    Ok(())
}

