//! AlphaIru
//! Twitch Reader
//!
//! twitch/handlers.rs
//! 
//! This is the module for Twitch API related functions,
//! and this one handles the server messages
//!

use twitch_irc::message::ServerMessage;
use crate::twitch::types::Message;

pub fn handle_server_message(
    message: ServerMessage,
) -> Option<Message> {
    match &message {
            ServerMessage::Join(msg) => {
                Some(Message::DM {
                    username: "[SYSTEM]".to_string(),
                    user_id: "0".to_string(),
                    message_id: None,
                    sent_ts: None,
                    msg: format!("Successfully joined: {}", msg.channel_login),
                    color: "#FFFF66".to_string(),
                    is_mod: false,
                    is_broadcaster: false,
                })
            }

            ServerMessage::Notice(msg) => {
                Some(Message::DM {
                    username: "[SYSTEM]".to_string(),
                    user_id: "0".to_string(),
                    message_id: None,
                    sent_ts: None,
                    msg: format!("Notice from Twitch: {}", msg.message_text),
                    color: "#FFFF66".to_string(),
                    is_mod: false,
                    is_broadcaster: false,
                })
            }

            ServerMessage::Privmsg(msg) => {
                Some(Message::DM {
                    username: if msg.sender.name.is_empty() {
                        msg.sender.login.clone()
                    } else {
                        msg.sender.name.clone()
                    },
                    user_id: msg.sender.id.clone(),
                    message_id: Some(msg.message_id.clone()),
                    sent_ts: Some(msg.server_timestamp.timestamp_millis()),
                    msg: msg.message_text.clone(),
                    color: msg.name_color
                        .as_ref()
                        .map(|c| c.to_string())
                        .unwrap_or_else(|| "#FFFFFF".to_string()),
                    is_mod: msg.badges.iter().any(|b| b.name == "moderator"),
                    is_broadcaster: msg.badges.iter().any(|b| b.name == "broadcaster"),
                })
            }
            _ => None
    }

}
