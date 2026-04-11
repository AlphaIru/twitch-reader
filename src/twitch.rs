//! AlphaIru
//! Twitch Reader
//!
//! twitch.rs
//! 
//! This is the module for Twitch API related functions.
//!

use std::fmt::Display;

use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::{ClientConfig, SecureTCPTransport, TwitchIRCClient};
use twitch_irc::message::ServerMessage;
use tokio::sync::{broadcast, mpsc};

use crate::ChatPayload;

#[derive(Debug, Clone)]
pub enum Message {
    DM {
        username: String,
        user_id: String,
        msg: String,
        color: String,
        is_mod: bool,        
        is_broadcaster: bool,
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Message::DM {
                username,
                user_id,
                msg,
                color,
                is_mod,
                is_broadcaster
            } => write!(
                f,
                "username: {} user_id: {} msg: {} color: {} is_mod: {} is_broadcaster: {}",
                username,
                user_id,
                msg,
                color,
                is_mod,
                is_broadcaster
            ),
        }
    }
}

pub fn connect(
    username: String,
    oauth_token: String,
    broadcast_tx: broadcast::Sender<ChatPayload>,
) {
    let (twitch_tx, mut twitch_rx) = mpsc::channel::<Message>(100);

    tokio::spawn(async move {
        run_twitch_listener(username, oauth_token, twitch_tx).await;
    });

    let tx_for_payload = broadcast_tx.clone();
    tokio::spawn(async move {
        while let Some(msg) = twitch_rx.recv().await {
            match msg {
                Message::DM { username, user_id, msg, color, is_mod, is_broadcaster } => {
                    let _ = tx_for_payload.send(ChatPayload {
                        username,
                        user_id,
                        msg,
                        color,
                        is_mod,
                        is_broadcaster,
                    });
                }
            }
        }
    });
}

pub async fn run_twitch_listener(
    username: String,
    oauth_token: String,
    tx: mpsc::Sender<Message>,
)
{
    let config = ClientConfig::new_simple(
        StaticLoginCredentials::new(username.clone(), Some(oauth_token))
    );

    let (mut incoming_messages, client) = 
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    client.join(username.clone()).expect("Failed to join channel");

    let _ = tx.send(Message::DM {
        username: "[SYSTEM]".to_string(),
        user_id: "0".to_string(),
        msg: format!("Joining chat in channel \"{}\". Twitch listener started!", username),
        color: "#FFFF66".to_string(),
        is_mod: false,
        is_broadcaster: false,
    }).await;

    while let Some(message) = incoming_messages.recv().await {

        match &message {
            ServerMessage::Join(msg) => {
                tx.send(Message::DM {
                    username: "[SYSTEM]".to_string(),
                    user_id: "0".to_string(),
                    msg: format!("Successfully joined: {}", msg.channel_login),
                    color: "#FFFF66".to_string(),
                    is_mod: false,
                    is_broadcaster: false,
                }).await.unwrap();
            }

            ServerMessage::Notice(msg) => {
                tx.send(Message::DM {
                    username: "[SYSTEM]".to_string(),
                    user_id: "0".to_string(),
                    msg: format!("Noice from Twitch: {}", msg.message_text),
                    color: "#FFFF66".to_string(),
                    is_mod: false,
                    is_broadcaster: false,
                }).await.unwrap();
            }
            
           _ => {} 
        }

        if let ServerMessage::Privmsg(message) = message {
            // println!("Got a message: {}", message.message_text);

            let color = message.name_color
                .map(|v| format!("{}", v))
                .unwrap_or_else(|| "#FFFFFF".to_string());

            let is_mod = message.badges.iter().any(|badge| badge.name == "moderator");
            let is_broadcaster = message.badges.iter().any(|badge| badge.name == "broadcaster");

            let _ = tx.send(Message::DM {
                username: message.sender.login.clone(),
                user_id: message.sender.id,
                msg: message.message_text.clone(),
                color,
                is_mod,
                is_broadcaster
            }).await;
        }
    } 
}
