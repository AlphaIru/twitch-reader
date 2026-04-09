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

#[derive(Clone, Debug)]
pub struct ChatPayload {
    pub username: String,
    pub user_id: String,
    pub msg: String,
    pub processed_msg: String
}

#[tokio::main]
async fn main() {

    dotenv().ok();
    
    let username = env::var("TWITCH_USERNAME")
        .expect("Error: .env file not found or TWITCH_USERNAME must be set");
    // println!("Username: {}", username);

    let oauth_token = env::var("TWITCH_OAUTH_TOKEN")
        .expect("Error: .env file not found or TWITCH_OAUTH_TOKEN must be set");
    // println!("Oauth Token: {}", oauth_token);

    let enable_yomi = env::var("ENABLE_YOMI").unwrap_or_else(|_| "false".to_string()) == "true"; 
    let enable_nico = env::var("ENABLE_NICO").unwrap_or_else(|_| "false".to_string()) == "true";


    let (broadcast_tx, _) = broadcast::channel::<ChatPayload>(16);
    let voice_queue_counter = Arc::new(AtomicI32::new(0));

    let tx_for_hub = broadcast_tx.clone();
    let voice_counter_for_hub = Arc::clone(&voice_queue_counter);
    tokio::spawn(async move {
        yomi::run_yomi_hub(username, oauth_token, tx_for_hub, voice_counter_for_hub).await;
    });

    let mut rx_for_log = broadcast_tx.subscribe();
    tokio::spawn(async move {
        while let Ok(payload) = rx_for_log.recv().await {
            println!("{} ({}): {}", payload.username, payload.user_id, payload.msg);
        }
    });

    if enable_yomi {
        let mut _rx_for_yomi = broadcast_tx.subscribe();
        let voice_counter_for_yomi = Arc::clone(&voice_queue_counter);
        tokio::spawn(async move {
            println!("Yomi (Voice) is active.");
            while let Ok(payload) = _rx_for_yomi.recv().await {
                if payload.processed_msg.trim() == "" {
                    voice_counter_for_yomi.fetch_add(-1, Ordering::Relaxed);
                    continue;
                }
                voice_creation::speak(payload.processed_msg).await;

                voice_counter_for_yomi.fetch_add(-1, Ordering::Relaxed);
            }
        });
    }

    if enable_nico {
        let mut _rx_for_nico = broadcast_tx.subscribe();
        tokio::spawn(async move {
            println!("Nico (WebSocket) is active.");
        });
    }

    println!("Twitch Reader System started!");
    tokio::signal::ctrl_c().await.unwrap();
}

