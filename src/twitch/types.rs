//! AlphaIru
//! Twitch Reader
//!
//! twitch.rs
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
        msg: String,
        color: String,
        is_mod: bool,        
        is_broadcaster: bool,
    },
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


