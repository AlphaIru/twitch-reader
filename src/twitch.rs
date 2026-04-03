/*
 * AlphaIru
 * Twitch Reader
 *
 * twitch.rs
 * 
 * This is the module for Twitch API related functions.
 *  
 */

use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::{ClientConfig, SecureTCPTransport, TwitchIRCClient};
use twitch_irc::message::ServerMessage;
use tokio::sync::mpsc;


pub async fn run_twitch_listener(
    username: String,
    oauth_token: String,
    tx: mpsc::Sender<String>,
)
{
    let config = ClientConfig::new_simple(
        StaticLoginCredentials::new(username.clone(), Some(oauth_token))
    );

    let (mut incoming_messages, client) = 
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    client.join(username.clone()).expect("Failed to join channel");

    println!("Joining chat in channel \"{}\"!", username);
    println!("Twitch listener started!");

    while let Some(message) = incoming_messages.recv().await {

        match &message {
            ServerMessage::Join(msg) => println!("Successfully joined: {}", msg.channel_login),
            ServerMessage::Notice(msg) => println!("Notice from Twitch: {}", msg.message_text),
            _ => {} 
        }

        if let ServerMessage::Privmsg(message) = message {
            // println!("Got a message: {}", message.message_text);
            let _ = tx.send(message.message_text).await;
        }
    } 
}
