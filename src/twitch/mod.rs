//! AlphaIru
//! Twitch Reader
//!
//! twitch/mod.rs
//! 
//! This is the module for Twitch API related functions,
//! and this one handles to connect other modules
//!

pub mod types;
pub mod handlers;
pub mod client;

use tokio::sync::{broadcast, mpsc, oneshot};
use crate::ChatPayload;
use self::types::Message;

pub fn connect(
    username: String,
    oauth_token: String,
    broadcast_tx: broadcast::Sender<ChatPayload>,
    narrowcast_rx: mpsc::Receiver<String>,
    config_tx: Option<oneshot::Sender<(String, String)>>,
) {
    let (twitch_tx, mut twitch_rx) = mpsc::channel::<Message>(100);

    tokio::spawn(async move {
        client::run_twitch_listener(
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
