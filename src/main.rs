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
use tokio::net::unix::pipe::Receiver;
use std::env;
use tokio::sync::mpsc;

mod twitch;
use crate::twitch::Message;

mod voice_creation;
use crate::voice_creation::speak;

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

    let (tx, mut rx) = mpsc::channel::<Message>(32);

    tokio::spawn(async move {
        twitch::run_twitch_listener(username, oauth_token, tx).await;
    });

    println!("Twitch Reader is running!");

    // dbg!(&dict);

    while let Some(msg) = rx.recv().await {

        let recv_msg = match msg {
            Message::DM { username, user_id, msg } => 
            {
                // println!("Username: {}", username);
                // println!("User_ID: {}", user_id);
                // println!("Message: {}", msg);

                format!("{}からメッセージ： {}", username, msg)
            }
        };

        println!("{}", recv_msg);

        let recv_msg = clean_text(
            recv_msg, 
            &dict_map,
            &dict_trie
        );

        println!("Pasered msg: {}", recv_msg);

        let speak_text = format!("{}", recv_msg);
        speak(&speak_text);
    }
}

