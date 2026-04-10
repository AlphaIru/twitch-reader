//! AlphaIru
//! Twitch Reader
//! 
//! This is a simple program that would receive
//! Twitch chat and handle in-and-out requests
//! through here.
//! 
//! main.rs
//!
//! This is the main entry point of the program and handles
//! the main loop of the program.
//!


use std::env;
use dotenvy::dotenv;
use std::sync::Arc;
use std::sync::atomic::{AtomicI32, Ordering};

use tokio::sync::broadcast;

mod yomi;
mod voice_creation;
mod word_process;
mod twitch;
mod tui;

#[derive(Clone, Debug, Default)]
pub struct ChatPayload {
    pub username: String,
    pub user_id: String,
    pub msg: String,
    pub processed_msg: String,
    pub color: String,
    pub is_mod: bool,
    pub is_broadcaster: bool,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    dotenv().ok();

    let username = env::var("TWITCH_USERNAME")
        .expect("Error: .env file not found or TWITCH_USERNAME must be set");
    let oauth_token = env::var("TWITCH_OAUTH_TOKEN")
        .expect("Error: .env file not found or TWITCH_OAUTH_TOKEN must be set");
    let enable_yomi = env::var("ENABLE_YOMI").unwrap_or_else(|_| "false".to_string()) == "true"; 
    let enable_nico = env::var("ENABLE_NICO").unwrap_or_else(|_| "false".to_string()) == "true";


    let (broadcast_tx, _) = broadcast::channel::<ChatPayload>(16);
    let voice_queue_counter = Arc::new(AtomicI32::new(0));

    let tx_for_hub = broadcast_tx.clone();
    let voice_counter_for_hub = Arc::clone(&voice_queue_counter);
    tokio::spawn(async move {
        yomi::run_yomi_hub(username, oauth_token, tx_for_hub, voice_counter_for_hub).await;
    });

    if enable_yomi {
        let mut _rx_for_yomi = broadcast_tx.subscribe();
        let voice_counter_for_yomi = Arc::clone(&voice_queue_counter);

        let tx_for_yomi = broadcast_tx.clone();
        let max_queue: i32 = env::var("MAX_QUEUE_COUNT")
            .unwrap_or_else(|_| "100".to_string())
            .parse()
            .unwrap_or(100);
        let query_policy = env::var("QUEUE_DROP_POLICY").unwrap_or_else(|_| "drop_new".to_string());

        tokio::spawn(async move {
            let _ = tx_for_yomi.send(ChatPayload {
                username: "SYSTEM".to_string(),
                user_id: "0".to_string(),
                msg: "Yomi (Voice) is active.".to_string(),
                color: "#FFFF66".to_string(),
                ..Default::default()
            });

            while let Ok(mut payload) = _rx_for_yomi.recv().await {

                if payload.username == "SYSTEM" || payload.username == "SKIP"
                {
                    continue;
                }
                if query_policy == "drop_old" {
                    while voice_counter_for_yomi.load(Ordering::SeqCst) > max_queue
                    {
                        if let Ok(next_payload) = _rx_for_yomi.try_recv(){
                            if next_payload.username == "SYSTEM" { continue; }

                            let _ = tx_for_yomi.send(ChatPayload {
                                username: "SYSTEM".to_string(),
                                user_id: "0".to_string(),
                                msg: format!("Queue is full! Skipped Reading for {} ({}): {}", payload.username, payload.user_id, payload.msg),
                                color: "#FFFF66".to_string(),
                                ..Default::default()
                            });
                            voice_counter_for_yomi.fetch_add(-1, Ordering::Relaxed);
                            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                            payload= next_payload;
                        } 
                        else {
                            break;
                        }
                    }
                }

                if payload.processed_msg.trim().is_empty() {
                    voice_counter_for_yomi.fetch_add(-1, Ordering::Relaxed);
                    continue;
                }
                voice_creation::speak(payload.processed_msg).await;

                voice_counter_for_yomi.fetch_add(-1, Ordering::Relaxed);
            }
        });
    }

    if enable_nico {
        let tx_for_nico = broadcast_tx.clone();
        tokio::spawn(async move {
            let _ = tx_for_nico.send(ChatPayload {
                username: "SYSTEM".to_string(),
                user_id: "0".to_string(),
                msg: "Nico (WebSocket) is active.".to_string(),
                color: "#FFFF66".to_string(),
                ..Default::default()
            });
        });

        let mut _rx_for_nico = broadcast_tx.subscribe();
        tokio::spawn(async move {
        });
    }


    // TUI
    let _ = broadcast_tx.send(ChatPayload {
        username: "SYSTEM".to_string(),
        user_id: "0".to_string(),
        msg: "Twitch Reader System started.".to_string(),
        color: "#FFFF66".to_string(),
        ..Default::default()
    });
    // println!("Twitch Reader System started!");

    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    crossterm::execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;
    let backend = ratatui::backend::CrosstermBackend::new(stdout);
    let mut terminal = ratatui::Terminal::new(backend)?;

    let mut logs: Vec<String> = Vec::new();
    let mut rx_for_tui_log = broadcast_tx.subscribe();

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
            
            // logs.insert(0, log_entry);
            logs.push(log_entry);

            if logs.len() > 50 {
                logs.remove(0);
            }
        }

        terminal.draw(|f| tui::render(f, &logs))?;

        if crossterm::event::poll(std::time::Duration::from_millis(100))? {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                if key.code == crossterm::event::KeyCode::Esc {
                    break; 
                }
            }
        }
    }

    crossterm::execute!(std::io::stdout(), crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}

