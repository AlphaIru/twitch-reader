//! AlphaIru
//! Twitch Reader
//!
//! twitch.rs
//! 
//! This is the module for Twitch API related functions,
//! and this one handles the server messages
//!

use tokio::sync::oneshot;
use twitch_irc::message::ServerMessage;
use crate::twitch::types::Message;

pub fn handle_server_message(
    message: ServerMessage,
    config_tx_opt: &mut Option<oneshot::Sender<(String, String)>>,
) -> Option<Message> {
    match &message {
            ServerMessage::GlobalUserState(state) => {
                if let Some(tx) = config_tx_opt.take() {
                    let my_name = state.user_name.clone();
                    let my_color = state.name_color.as_ref()
                        .map(|c| c.to_string())
                        .unwrap_or_else(|| "#FFFFFF".to_string());

                    let _ = tx.send((my_name, my_color));
                }
                None
            }
            

            ServerMessage::Join(msg) => {
                Some(Message::DM {
                    username: "[SYSTEM]".to_string(),
                    user_id: "0".to_string(),
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
                    msg: format!("Notice from Twitch: {}", msg.message_text),
                    color: "#FFFF66".to_string(),
                    is_mod: false,
                    is_broadcaster: false,
                })
            }

            _ => None
        }

}
