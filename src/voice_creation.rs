//! AlphaIru
//! Twitch Reader
//!
//! voice_creation.rs 
//!
//! This is the module for voice creation related functions.
//!


use std::process::{Command, Stdio};
use std::io::Write;
use std::env;


pub fn speak(text: &str) {

    let dict_path = env::var("OPEN_JTALK_DICT_PATH")
        .expect("Error: .env file not found or OPEN_JTALK_DICT_PATH must be set");

    let voice_path = env::var("OPEN_JTALK_VOICE_PATH")
        .expect("Error: .env file not found or OPEN_JTALK_VOICE_PATH must be set");

    let mut child = Command::new("open_jtalk")
        .args(&[
            "-x", &dict_path,
            "-m", &voice_path,
            "-ow", "/dev/stdout",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start open_jtalk");
    
    /*
    match(child.stdin.take()) {
        Ok(mut stdin) => {
            stdin.write_all(text.as_bytes()).ok();
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    */

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(text.as_bytes()).ok();
    }


    if let Some(stdout) = child.stdout.take() {
        Command::new("pw-play")
            .arg("-")
            .stdin(stdout)
            .status()
            .ok();
    }
}
