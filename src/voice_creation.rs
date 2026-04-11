//! AlphaIru
//! Twitch Reader
//!
//! voice_creation.rs 
//!
//! This is the module for voice creation related functions.
//!


use std::process::Stdio;
use std::env;

use tokio::process::Command;
use tokio::io::AsyncWriteExt;


pub async fn speak(text: String) -> bool {
    if text.is_empty() {
        return false;
    }

    let dict_path = env::var("OPEN_JTALK_DICT_PATH")
        .expect("Error: .env file not found or OPEN_JTALK_DICT_PATH must be set");

    let voice_path = env::var("OPEN_JTALK_VOICE_PATH")
        .expect("Error: .env file not found or OPEN_JTALK_VOICE_PATH must be set");

    let mut child = Command::new("open_jtalk")
        .args([
            "-r", "1.25",  
            "-x", &dict_path,
            "-m", &voice_path,
            "-ow", "/dev/stdout",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start open_jtalk");
    

    if let Some(mut stdin) = child.stdin.take() {
        let _ = stdin.write_all(text.as_bytes()).await;
    }


    if let Some(stdout) = child.stdout.take() {
        let std_stdout: Stdio = stdout.try_into().expect("Failed to get stdout");

        let status = Command::new("paplay")
            .arg("--device=TwitchReader")
            .arg("--raw")                
            .arg("--channels=1")
            .arg("--rate=48000")
            .stdin(std_stdout)
            .status()
            .await;

        if status.is_err() {
            return false;
            // println!("Error: {}", err);
        }
    }

    true
}
