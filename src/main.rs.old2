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

use tokio::sync::broadcast;

mod nico;
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
    let enable_nico = env::var("ENABLE_NICO").unwrap_or_else(|_| "false".to_string()) == "true";


    let (broadcast_tx, _) = broadcast::channel::<ChatPayload>(16);

    twitch::connect(
        username.clone(),
        oauth_token.clone(),
        broadcast_tx.clone()
    );

    if enable_nico {
        let tx_for_nico = broadcast_tx.clone();
        tokio::spawn(async move {
            nico::start_nico_server(tx_for_nico).await;
        });
    }


    tui::run_tui(broadcast_tx)?;

    Ok(())
}

