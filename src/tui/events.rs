//! AlphaIru
//! Twitch Reader
//! 
//! tui/events.rs
//!
//! This holds the events
//! of the tui.
//!     

use tokio::sync::mpsc;

use crossterm::event::{KeyCode, KeyEvent};
use crate::tui::state::{AppState, InputMode};


pub fn handle_normal(
    key: KeyEvent,
    app_state: &mut AppState
) {
    match key.code {
        KeyCode::Char('i') => {
            app_state.mode = InputMode::Insert;
        }
        KeyCode::Char(':') => {
            app_state.mode = InputMode::Command;
            app_state.input_text.clear();
        }
        KeyCode::Char('k') | KeyCode::Up => {
            app_state.scroll_offset = app_state.scroll_offset.saturating_add(1);
        }
        KeyCode::Char('j') | KeyCode::Down => {
            app_state.scroll_offset = app_state.scroll_offset.saturating_sub(1);
        }
        KeyCode::Char('G') => {
            app_state.scroll_offset = 0;
        }
        KeyCode::Char('h') => {
            if ! app_state.show_help {
                app_state.show_help = true;
            }
        }
        _ => (),
    }
}

pub async fn handle_insert(
    key: KeyEvent,
    app_state: &mut AppState,
    narrowcast_tx: mpsc::Sender<String>
) {
    match key.code {
        KeyCode::Esc => {
            app_state.mode = InputMode::Normal;
        }
        KeyCode::Char(c) => {
            app_state.input_text.push(c);
        }
        KeyCode::Backspace => {
            app_state.input_text.pop();
        }
        KeyCode::Enter => {
            if !app_state.input_text.is_empty() {
                let _ = narrowcast_tx.send(app_state.input_text.clone()).await;
                
                let self_log = format!(
                    "{}|false|true|{}: {}",
                    app_state.my_color,
                    app_state.my_name,
                    app_state.input_text
                );
                app_state.logs.push(self_log);

                if app_state.logs.len() > 100 {
                    app_state.logs.remove(0);
                }
                if app_state.input_text == "/clear" {
                    app_state.logs.clear();
                }

                app_state.input_text.clear();
            }
            app_state.mode = InputMode::Normal;
        }
        _ => (),
    }
}


pub async fn handle_command(
    key: KeyEvent,
    app_state: &mut AppState,
    narrowcast_tx: mpsc::Sender<String>
) -> bool {
    match key.code {
        KeyCode::Esc => {
            app_state.mode = InputMode::Normal;
        }
        KeyCode::Char(c) => {
            app_state.input_text.push(c);
        }
        KeyCode::Backspace => {
            app_state.input_text.pop();
        }
        KeyCode::Enter => {
            match app_state.input_text.as_str() {
                "q" | "quit" => return false,
                "clear" => {
                    let _ = narrowcast_tx.send("/clear".to_string()).await;
                    app_state.logs.clear()
                }
                _ => {}
            }
            app_state.input_text.clear();
            app_state.mode = InputMode::Normal;
        }
        _ => (),
    }

    true
}


pub async fn handle_key(
    key: KeyEvent,
    app_state: &mut AppState,
    narrowcast_tx: mpsc::Sender<String>
) -> bool {
    if app_state.show_help {
        if let KeyCode::Char('q') | KeyCode::Char('h') | KeyCode::Esc = key.code {
            app_state.show_help = false;
            return true
        }
        return true
    }

    match app_state.mode {

        InputMode::Normal => {
            handle_normal(key, app_state);
            true
        }

        InputMode::Insert => {
            handle_insert(key, app_state, narrowcast_tx).await;
            true
        } 
        InputMode::Command => {
            handle_command(key, app_state, narrowcast_tx).await
        }
    }
}
