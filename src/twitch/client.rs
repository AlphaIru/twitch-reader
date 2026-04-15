//! AlphaIru
//! Twitch Reader
//!
//! twitch.rs
//! 
//! This is the module for Twitch API related functions,
//! and this one handles the loops
//!

use tokio::sync::{mpsc, oneshot};
use twitch_irc::{
    ClientConfig, 
    SecureTCPTransport, 
    TwitchIRCClient, 
    login::StaticLoginCredentials,
    message::IRCMessage
};

use crate::twitch::types::Message;
use crate::twitch::handlers::handle_server_message;

pub async fn run_twitch_listener(
    username: String,
    oauth_token: String,
    tx: mpsc::Sender<Message>,
    mut narrowcast_rx: mpsc::Receiver<String>,
    mut config_tx: Option<oneshot::Sender<(String, String)>>,
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
        while let Some(msg_to_send) = narrowcast_rx.recv().await {
            let msg = msg_to_send.trim();
            if msg.is_empty() {
                continue;
            }
            match msg {
                "/clear" => {
                    let raw = format!("CLEARCHAT #{}", channel_name.clone());
                    if let Ok(irc_msg) = IRCMessage::parse(&raw) {
                       let _ = client_clone.send_message(irc_msg).await;
                    }

                }
                _ => {let _ = client_clone.say(channel_name.clone(), msg.to_string()).await;}
            }
        }
    });

    while let Some(message) = incoming_messages.recv().await {
        if let Some(msg) = handle_server_message(message,&mut config_tx) {
            let _ = tx.send(msg).await;
        }
    }
}
