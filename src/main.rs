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
use std::sync::atomic::AtomicUsize;

use tokio::sync::broadcast;

mod yomi;
mod word_process;
mod voice_creation;
mod twitch;
mod tui;

#[derive(Clone, Debug, Default)]
pub struct ChatPayload {
    pub username: String,
    pub user_id: String,
    pub msg: String,
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

    let (broadcast_tx, _) = broadcast::channel::<ChatPayload>(16);

    twitch::connect(
        username.clone(),
        oauth_token.clone(),
        broadcast_tx.clone()
    );

    if enable_yomi {
        let rx_for_yomi = broadcast_tx.subscribe();
        let tx_for_yomi = broadcast_tx.clone();
        let voice_queue_counter = Arc::new(AtomicUsize::new(0));

        tokio::spawn(async move {
            yomi::start_reading(
                rx_for_yomi,
                tx_for_yomi,
                voice_queue_counter.clone()
        ).await;
        });
    }


    tui::run_tui(broadcast_tx)?;

    Ok(())
}

