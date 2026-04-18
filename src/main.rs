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

use tokio::sync::{broadcast, mpsc, oneshot};

mod auth;
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

    let oauth_token = auth::authenticate().await?.access_token;

    let (config_tx, config_rx) = oneshot::channel::<(String, String)>();
    let (broadcast_tx, _) = broadcast::channel::<ChatPayload>(16);
    let (narrowcast_tx, narrowcast_rx) = mpsc::channel::<String>(100);

    twitch::connect(
        username.clone(),
        oauth_token,
        broadcast_tx.clone(),
        narrowcast_rx,
        Some(config_tx)
    );

    tui::run_tui(broadcast_tx, narrowcast_tx, config_rx).await?;

    Ok(())
}

