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
use std::sync::atomic::{AtomicUsize, Ordering};

use tokio::sync::{broadcast, mpsc};

use crate::ChatPayload;
use crate::word_process::{
    clean_text,
    load_files,
    limit_length
};
use crate::voice_creation::speak;


pub fn get_env_variables() -> (bool, usize, usize, String) {
    let verbose = env::var("VERBOSE_LOG").unwrap_or_else(|_| "false".to_string()) == "true";

    let max_chars: usize = env::var("MAX_CHAR_COUNT")
        .unwrap_or_else(|_| "100".to_string())
        .parse()
        .unwrap_or(100);
    let max_queue: usize = env::var("MAX_QUEUE_COUNT")
        .unwrap_or_else(|_| "100".to_string())
        .parse()
        .unwrap_or(100);
    
    let query_policy = env::var("QUEUE_DROP_POLICY").unwrap_or_else(|_| "drop_new".to_string());

    (verbose, max_chars, max_queue, query_policy)
} 


pub async fn should_process_msg(
    rx: &mut broadcast::Receiver<ChatPayload>,
    tx: &broadcast::Sender<ChatPayload>,
    payload: &mut ChatPayload,
    queue_counter: &Arc<AtomicUsize>,
    verbose: bool,
    max_queue: usize,
    query_policy: String
) -> bool {
    if query_policy != "drop_old" {
        
        if verbose {
            let _ = tx.send(ChatPayload {
                username: "[SYSTEM]".to_string(),
                user_id: "0".to_string(),
                msg: format!("Queue is full. Skipped {}'s message.", payload.username).to_string(),
                color: "#FFFF66".to_string(),
                ..Default::default()
            });
        }
        return false;
    }

    while queue_counter.load(Ordering::SeqCst) >= max_queue {
        while let Ok(next_payload) = rx.try_recv() {
            if next_payload.username == "[SYSTEM]" || next_payload.username == "[SKIP]" {
                continue;
            }
            if verbose {
                let _ = tx.send(ChatPayload {
                    username: "[SYSTEM]".to_string(),
                    user_id: "0".to_string(),
                    msg: format!("Queue is full. Skipped {}'s message.", next_payload.username).to_string(),
                    color: "#FFFF66".to_string(),
                    ..Default::default()
                });
            }
            *payload = next_payload;
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
    true
}


pub async fn start_reading(
    mut rx: broadcast::Receiver<ChatPayload>,
    tx: broadcast::Sender<ChatPayload>,
    queue_counter: Arc<AtomicUsize>,
) {
    let (dict_map, dict_trie) = load_files();

    let (speak_tx, mut speak_rx) = mpsc::channel::<String>(100);

    let queue_counter_clone = queue_counter.clone();
    tokio::spawn(async move {
        while let Some(msg) = speak_rx.recv().await {
            speak(msg).await;
            queue_counter_clone.fetch_sub(1, Ordering::SeqCst);
        }
    });

    let (verbose, max_chars, max_queue, query_policy) = get_env_variables();
    let mut last_user_id: Option<String> = None;

    if verbose {
        let _ = tx.send(ChatPayload {
            username: "[SYSTEM]".to_string(),
            user_id: "0".to_string(),
            msg: format!("[VOICE SETTINGS] max_queue: {}, max_chars: {}, query_policy: {}", max_queue, max_chars, query_policy).to_string(),
            color: "#FFBB66".to_string(),
            ..Default::default()
        });
    }

    let _ = tx.send(ChatPayload {
        username: "[SYSTEM]".to_string(),
        user_id: "0".to_string(),
        msg: "Yomi (Voice) is active.".to_string(),
        color: "#FFFF66".to_string(),
        ..Default::default()
    });

    while let Ok(mut payload) = rx.recv().await {

        if payload.username == "[SYSTEM]" || payload.username == "[SKIP]"
        {
            continue;
        }

        if !should_process_msg(
            &mut rx,
            &tx,
            &mut payload,
            &queue_counter,
            verbose,
            max_queue,
            query_policy.clone()
        ).await 
        {
            continue;
        }

        let display_msg = if last_user_id == Some(payload.user_id.clone()) {
            payload.msg.clone()
        }
        else {
            last_user_id = Some(payload.user_id.clone());
            format!("{}: {}", payload.username, payload.msg)
        };

        let processed_msg = clean_text(
            display_msg,
            &dict_map,
            &dict_trie,
        );
        let processed_msg = limit_length(processed_msg, max_chars);

        if processed_msg.trim().is_empty() {
            continue;
        }

        queue_counter.fetch_add(1, Ordering::SeqCst);
        if speak_tx.send(processed_msg.clone()).await.is_err() {
            queue_counter.fetch_sub(1, Ordering::SeqCst);
            continue;
        }

    }
}

