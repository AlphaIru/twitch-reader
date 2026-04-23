//! AlphaIru
//! Twitch Reader
//!
//! twitch/types.rs
//! 
//! This is the module for Twitch API related functions
//! and this one handles the types
//!

use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Message {
    DM {
        username: String,
        user_id: String,
        
        message_id: Option<String>,
        sent_ts: Option<i64>,

        msg: String,
        color: String,
        
        is_mod: bool,        
        is_broadcaster: bool,
    },
}


#[derive(Clone, Debug, Default)]
pub struct ChatPayload {
    pub username: String,
    pub user_id: String,

    pub message_id: Option<String>,
    pub sent_ts: Option<i64>,

    pub msg: String,
    pub color: String,

    pub is_mod: bool,
    pub is_broadcaster: bool,
}


#[derive(Debug, Clone)]
pub enum Outgoing {
    Chat(String),
    Clear,
}


impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Message::DM {
                username,
                user_id,
                message_id,
                sent_ts,
                msg,
                color,
                is_mod,
                is_broadcaster
            } => write!(
                f,
                "username: {} user_id: {} msg: {} message_id: {} sent_ts: {} color: {} is_mod: {} is_broadcaster: {}",
                username,
                user_id,
                message_id.as_deref().unwrap_or("-"),
                sent_ts
                    .map(|ts| ts.to_string())
                    .unwrap_or_else(|| "-".to_string()),
                msg,
                color,
                is_mod,
                is_broadcaster
            ),
        }
    }
}


