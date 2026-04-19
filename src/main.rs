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

use twitch_api::helix::HelixClient;
use twitch_api::twitch_oauth2::{AccessToken, UserToken};

mod auth;
mod helix;
mod twitch;
mod tui;

use auth::authenticate;

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

    let oauth_token = authenticate().await?.access_token;

    let client: HelixClient<reqwest::Client> = HelixClient::default();
    let token = UserToken::from_token(
        &client,
        AccessToken::from(oauth_token.clone()),
    )
    .await?;

    let user = client
        .get_user_from_login(&username, &token)
        .await?
        .ok_or("User not found")?;

    println!("Helix OK!");
    println!("id          = {}", user.id);
    println!("login       = {}", user.login);
    println!("displayname = {}", user.display_name);

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

    // tui::run_tui(broadcast_tx, narrowcast_tx, config_rx).await?;

    // Debug
    tokio::signal::ctrl_c().await?;

    Ok(())
}

