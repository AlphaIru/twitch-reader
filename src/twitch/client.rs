//! AlphaIru
//! Twitch Reader
//!
//! twitch/client.rs
//! 
//! This is the module for Twitch API related functions,
//! and this one handles the loops
//!

use tokio::sync::mpsc;
use twitch_irc::{
    ClientConfig, 
    SecureTCPTransport, 
    TwitchIRCClient, 
    login::StaticLoginCredentials, 
};
use twitch_api::twitch_oauth2::UserToken;

use crate::twitch::types::{Message, Outgoing};
use crate::twitch::handlers::handle_server_message;
use crate::helix::{
    moderation::clear_chat,
    types::CommandConfig,
};


pub async fn run_twitch_listener(
    username: String,
    oauth_token: String,
    helix_token: UserToken,
    tx: mpsc::Sender<Message>,
    mut narrowcast_rx: mpsc::Receiver<Outgoing>,
    command_config: CommandConfig,
) {
    let config = ClientConfig::new_simple(
        StaticLoginCredentials::new(username.clone(), Some(oauth_token))
    );

    let (mut incoming_messages, client) = 
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    client.join(username.clone()).expect("Failed to join channel");

    let client_clone = client.clone();
    let channel_name = username.clone();

    tokio::spawn(async move {
        while let Some(outgoing) = narrowcast_rx.recv().await {
            match outgoing {
                Outgoing::Clear => {
                    if let Err(e) = clear_chat(
                        &command_config.broadcaster_id,
                        &command_config.moderator_id,
                        &helix_token,
                    ).await {
                        eprintln!("Failed to clear chat via helix: {}", e);
                    }
                }
                Outgoing::Chat(msg) => {
                    let msg = msg.trim();
                    if msg.is_empty() {
                        continue;
                    }
                    let _ = client_clone
                        .say(channel_name.clone(), msg.to_string())
                        .await;
                }
            }
        }
    });


    while let Some(message) = incoming_messages.recv().await {
        if let Some(msg) = handle_server_message(message) {
            let _ = tx.send(msg).await;
        }
    }
}
