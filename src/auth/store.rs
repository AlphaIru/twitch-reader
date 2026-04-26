//! AlphaIru
//! Twitch Reader
//!
//! auth/store.rs
//! 
//! This is the module for twitch authentication,
//! and this one handles the storage of the tokens
//!

use std::fs;
use std::env;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AuthStore {
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
}


fn auth_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    Ok(PathBuf::from(
        env::var("AUTH_FILE").expect("AUTH_FILE must be set!"),
    ))
}


pub fn save_token(
    access_token: Option<&str>,
    refresh_token: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    
    let path = auth_path()?;

    let data = AuthStore {
        access_token: access_token.map(|s| s.to_string()),
        refresh_token: refresh_token.map(|s| s.to_string()),
    };

    let toml_path = toml::to_string_pretty(&data)?;
    fs::write(path, toml_path)?;

    Ok(())
}

pub fn load_token() -> Option<String> {
    
    let path = auth_path().ok()?;

    if !path.exists() {
        return None;
    }

    let content = fs::read_to_string(path).ok()?;
    let data: AuthStore = toml::from_str(&content).ok()?;

    data.refresh_token
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}
