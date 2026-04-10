//! AlphaIru
//! Twitch Reader
//! 
//! This is a simple program that would read out
//! Twitch chat messages to Open_JTalk in real-time.
//! 
//! yomi.rs
//!
//! This is the main entry point of the yomi
//! program and handles the program.
//!


use std::env;
use std::sync::Arc;
use std::sync::atomic::AtomicI32;

use tokio::sync::{broadcast, mpsc};

use crate::ChatPayload;
use crate::twitch::Message;
use crate::word_process::{
    clean_text,
    load_files,
    limit_length
};


pub async fn run_yomi_hub(
    username: String,
    oauth_token: String,
    broadcast_tx: broadcast::Sender<ChatPayload>,
    voice_queue_counter: Arc<AtomicI32>
) {

    let (dict_map, dict_trie) = load_files();
    let (twitch_tx, mut twitch_rx) = mpsc::channel::<Message>(100);


    // Turn on Twitch Listener task
    tokio::spawn(async move {
        crate::twitch::run_twitch_listener(
            username,
            oauth_token,
            twitch_tx
        ).await;
    });


    let max_chars: usize = env::var("MAX_CHAR_COUNT")
        .unwrap_or_else(|_| "100".to_string())
        .parse()
        .unwrap_or(100);

    if max_chars <= 0 {
        panic!("MAX_CHAR_COUNT must be greater than 0!");
    }


    let max_queue: i32 = env::var("MAX_QUEUE_COUNT")
        .unwrap_or_else(|_| "100".to_string())
        .parse()
        .unwrap_or(100);

    if max_queue <= 0 {
        panic!("MAX_QUEUE must be greater than 0!");
    }

    let max_queue_ciel = max_queue * 2;

    let drop_policy = env::var("QUEUE_DROP_POLICY")
        .unwrap_or_else(|_| "drop_new".to_string());

    let mut last_user_id: Option<String> = None;

    let _ = broadcast_tx.send(ChatPayload {
        username: "SYSTEM".to_string(),
        user_id: "0".to_string(),
        msg: format!("Twitch Reader is running!"),
        ..Default::default()
    });

    // Recv loop
    while let Some(msg) = twitch_rx.recv().await {
        let current_queue_num = voice_queue_counter.load(std::sync::atomic::Ordering::SeqCst);
        
        let Message::DM { 
            username,
            user_id,
            msg,
            color,
            is_mod,
            is_broadcaster
        } = msg;

        let should_process = if drop_policy == "drop_old" {
            current_queue_num < max_queue_ciel
        }
        else {
            current_queue_num < max_queue
        };

        if !should_process {
            let _ = broadcast_tx.send(ChatPayload {
                username: "SYSTEM".to_string(),
                user_id: "0".to_string(),
                msg: format!("Queue is full! ({} / {}): Skipped Reading for {}", current_queue_num, max_queue, msg),
                ..Default::default()
            });
            continue;
        }

        voice_queue_counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

        let display_text = if last_user_id.as_deref() == Some(&user_id) {
            msg.clone()
        } else {
            let f = format!("{}： {}", username, msg);
            last_user_id = Some(user_id.clone());
            f
        };

        let mut processed = limit_length(display_text, max_chars);
        processed = clean_text(
            processed, 
            &dict_map,
            &dict_trie
        );

        let payload = ChatPayload {
            username: username.clone(),
            user_id: user_id.clone(),
            msg: msg.clone(),
            processed_msg: processed,
            color: color.clone(),
            is_mod,
            is_broadcaster,
        };

        let _ = broadcast_tx.send(payload);
    }
}

