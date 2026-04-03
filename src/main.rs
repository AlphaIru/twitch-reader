/*
 * AlphaIru
 * Twitch Reader
 *
 * This is a simple program that would read out
 * Twitch chat messages to Open_JTalk in real-time.
 * 
 * main.rs
 *
 * This is the main entry point of the program and handles
 * the main loop of the program.
 *
 * */

mod twitch;

use dotenvy::dotenv;
use std::env;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    let username = env::var("TWITCH_USERNAME")
        .expect("Error: .env file not found or TWITCH_USERNAME must be set");
    // println!("Username: {}", username);

    let oauth_token = env::var("TWITCH_OAUTH_TOKEN")
        .expect("Error: .env file not found or TWITCH_OAUTH_TOKEN must be set");
    // println!("Oauth Token: {}", oauth_token);

    let (tx, mut rx) = mpsc::channel::<String>(32);

    tokio::spawn(async move {
        twitch::run_twitch_listener(username, oauth_token, tx).await;
    });

    println!("Twitch Reader is running!");

    while let Some(msg) = rx.recv().await {
        println!("Received message: {}", msg);
    }
}

