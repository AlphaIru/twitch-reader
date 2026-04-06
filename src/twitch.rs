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
use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub enum Message {
    DM {
        username: String,
        user_id: String,
        msg: String,
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Message::DM { username, user_id, msg } => write!(f, "username: {} user_id: {} msg: {}", username, user_id, msg),
        }
    }
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

    println!("Joining chat in channel \"{}\"!", username);
    println!("Twitch listener started!");

    while let Some(message) = incoming_messages.recv().await {

        match &message {
            ServerMessage::Join(msg) => println!("Successfully joined: {}", msg.channel_login),
            ServerMessage::Notice(msg) => println!("Notice from Twitch: {}", msg.message_text),
           _ => {} 
        }

        if let ServerMessage::Privmsg(message) = message {
            // println!("Got a message: {}", message.message_text);
            let _ = tx.send(Message::DM {
                username: message.sender.login.clone(),
                user_id: message.sender.id,
                msg: message.message_text.clone(),
            }).await;
        }
    } 
}
