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

use tokio::sync::{broadcast, mpsc};

use twitch_api::twitch_oauth2::{AccessToken, UserToken};

mod auth;
mod helix;
mod twitch;
mod tui;


use auth::authenticate;
use helix::{
    user::get_broadcaster_profile,
    types::CommandConfig
};
use twitch::types::{
    ChatPayload,
    Outgoing
};

use crate::helix::client::make_helix_client;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    dotenv().ok();

    let username = env::var("TWITCH_USERNAME")
        .expect("Error: .env file not found or TWITCH_USERNAME must be set");

    let oauth_token = authenticate().await?.access_token;

    let client = make_helix_client();
    let helix_token = UserToken::from_token(
        &client,
        AccessToken::from(oauth_token.clone()),
    ).await?;

    let broadcaster_profile = get_broadcaster_profile(
        &client,
        &helix_token,
        &username
    ).await?;

    let command_config = CommandConfig {
        broadcaster_login: broadcaster_profile.login.clone(),
        broadcaster_id: broadcaster_profile.id.clone(),
        moderator_id: helix_token.user_id.to_string(),
    };


    let (broadcast_tx, _) = broadcast::channel::<ChatPayload>(16);
    let (narrowcast_tx, narrowcast_rx) = mpsc::channel::<Outgoing>(100);

    twitch::connect(
        username.clone(),
        oauth_token,
        helix_token,
        broadcast_tx.clone(),
        narrowcast_rx,
        command_config,
    );

    tui::run_tui(
        broadcast_tx,
        narrowcast_tx,
        broadcaster_profile
    ).await?;

    // Debug
    // tokio::signal::ctrl_c().await?;

    Ok(())
}

