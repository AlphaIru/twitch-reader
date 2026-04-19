//! helix/user.rs

use std::env;

use serde::Deserialize;
use twitch_api::twitch_oauth2::UserToken;

use crate::helix::client::TwitchHelix;

#[derive(Debug, Clone)]
pub struct BroadcasterProfile {
    pub id: String,
    pub login: String,
    pub display_name: String,
    pub color: String,
}

#[derive(Debug, Deserialize)]
struct ChatColorResponse {
    data: Vec<ChatColorEntry>
}

#[derive(Debug, Deserialize)]
struct ChatColorEntry {
    user_id: String,
    user_login: String,
    user_name: String,
    color: String
}

pub async fn get_broadcaster_profile(
    client: &TwitchHelix,
    token: &UserToken,
    login: &str,
) -> Result<BroadcasterProfile, Box<dyn std::error::Error>> {
    let user = client
        .get_user_from_login(login, token)
        .await?
        .ok_or("User not found!")?;

    let color = get_broadcaster_color(
        user.id.as_ref(),
        token,
    ).await?;

    Ok(BroadcasterProfile {
        id: user.id.to_string(),
        login: user.login.to_string(),
        display_name: user.display_name.to_string(),
        color,
    })
}


async fn get_broadcaster_color(
    user_id: &str,
    token: &UserToken,
) -> Result<String, Box<dyn std::error::Error>> {
    let client_id = env::var("TWITCH_CLIENT_ID")
        .expect("Error: .env file not found or TWITCH_CLIENT_ID must be set");

    let access_token = &token.access_token.secret();

    let url = format!(
        "https://api.twitch.tv/helix/chat/color?user_id={}",
        user_id,
    );

    let response = reqwest::Client::new()
        .get(url)
        .header("Client-Id", client_id)
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?
        .error_for_status()?
        .json::<ChatColorResponse>()
        .await?;

    let color = response
        .data
        .into_iter()
        .next()
        .map(|entry| {
            let _ = (&entry.user_id, &entry.user_login, &entry.user_name);
            if entry.color.trim().is_empty() {
                "#FFFFFF".to_string()
            } else {
                entry.color
            }
        })
        .unwrap_or_else(|| "#FFFFFF".to_string());

    Ok(color)
}

