//! AlphaIru
//! Twitch Reader
//! 
//! This is a simple program that would read out
//! Twitch chat messages to Open_JTalk in real-time.
//! 
//! main.rs
//!
//! This is the main entry point of the program and handles
//! the main loop of the program.
//!


use dotenvy::dotenv;
use std::env;
use std::sync::Arc;
use std::sync::atomic::AtomicI32;

use tokio::sync::mpsc;

mod twitch;
use crate::twitch::Message;

mod voice_creation;

mod word_process;
use crate::word_process::{
    clean_text,
    load_files
};


#[tokio::main]
async fn main() {
    dotenv().ok();
    
    let (dict_map, dict_trie) = load_files();

    let username = env::var("TWITCH_USERNAME")
        .expect("Error: .env file not found or TWITCH_USERNAME must be set");
    // println!("Username: {}", username);

    let oauth_token = env::var("TWITCH_OAUTH_TOKEN")
        .expect("Error: .env file not found or TWITCH_OAUTH_TOKEN must be set");
    // println!("Oauth Token: {}", oauth_token);

    let (twitch_tx, mut twitch_rx) = mpsc::channel::<Message>(32);
    let (voice_tx, mut voice_rx) = mpsc::channel::<String>(100);

    let queue_counter = Arc::new(AtomicI32::new(0));
    let queue_counter_playing = Arc::clone(&queue_counter);

    tokio::spawn(async move {
        twitch::run_twitch_listener(
            username,
            oauth_token,
            twitch_tx
        ).await;
    });

    tokio::spawn(async move {
        while let Some(msg) = voice_rx.recv().await {
            // println!("Playing now: Queue counter: {}", queue_counter_playing.load(std::sync::atomic::Ordering::Relaxed));
            voice_creation::speak(msg).await;
            queue_counter_playing.fetch_add(-1, std::sync::atomic::Ordering::Relaxed);
        }
    });

    println!("Twitch Reader is running!");

    // dbg!(&dict);

    while let Some(msg) = twitch_rx.recv().await {
        queue_counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        // println!("Queue counter: {}", queue_counter.load(std::sync::atomic::Ordering::Relaxed));

        let raw_msg = match msg {
            Message::DM { username, user_id, msg } => 
            {
                // println!("Username: {}", username);
                // println!("User_ID: {}", user_id);
                // println!("Message: {}", msg);

                format!("{}： {}", username, msg)
            }
        };

        println!("{}", raw_msg);

        let processed_msg = clean_text(
            raw_msg, 
            &dict_map,
            &dict_trie
        );

        // println!("Parsed msg: {}", recv_msg);
        
        let _ = voice_tx.send(processed_msg).await;
    }
}

