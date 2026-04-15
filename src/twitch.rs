//! AlphaIru
//! Twitch Reader
//!
//! twitch.rs
//! 
//! This is the module for Twitch API related functions.
//!

use std::fmt::Display;

use twitch_irc::{
    ClientConfig,
    SecureTCPTransport,
    TwitchIRCClient,
    login::StaticLoginCredentials,
    message::{ServerMessage, IRCMessage},
};
use tokio::sync::{broadcast, mpsc, oneshot};

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
    narrowcast_rx: mpsc::Receiver<String>,
    config_tx: oneshot::Sender<(String, String)>,
) {
    let (twitch_tx, mut twitch_rx) = mpsc::channel::<Message>(100);

    tokio::spawn(async move {
        run_twitch_listener(
            username,
            oauth_token,
            twitch_tx,
            narrowcast_rx,
            config_tx
        ).await;
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
    mut narrowcast_rx: mpsc::Receiver<String>,
    config_tx: oneshot::Sender<(String, String)>,
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

            if msg.starts_with('/') {
                let raw_command = format!("PRIVMSG #{} :{}", channel_name, msg);

                if let Ok(irc_msg) = IRCMessage::parse(&raw_command) {
                    let _ = client_clone.send_message(irc_msg).await;
                }
            } else {
                let _ = client_clone.say(channel_name.clone(), msg.to_string()).await;
            }
        }
    });

    let _ = tx.send(Message::DM {
        username: "[SYSTEM]".to_string(),
        user_id: "0".to_string(),
        msg: format!("Joining chat in channel \"{}\". Twitch listener started!", username),
        color: "#FFFF66".to_string(),
        is_mod: false,
        is_broadcaster: false,
    }).await;

    let mut config_tx_opt = Some(config_tx);

    while let Some(message) = incoming_messages.recv().await {

        match &message {
            ServerMessage::GlobalUserState(state) => {
                if let Some(tx) = config_tx_opt.take() {
                    let my_name = state.user_name.clone();
                    let my_color = state.name_color.as_ref()
                        .map(|c| c.to_string())
                        .unwrap_or_else(|| "#FFFFFF".to_string());

                    let _ = tx.send((my_name, my_color));
                }
            }
            

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
                    msg: format!("Notice from Twitch: {}", msg.message_text),
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
                username: if message.sender.name.is_empty()
                { 
                    message.sender.login.clone() 
                }
                else
                {
                    message.sender.name.clone()
                },
                user_id: message.sender.id,
                msg: message.message_text.clone(),
                color,
                is_mod,
                is_broadcaster
            }).await;
        }
    } 
}
