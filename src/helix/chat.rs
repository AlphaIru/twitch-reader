//! helix/chat.rs

use std::env;

use serde::{Deserialize, Serialize};
use twitch_api::twitch_oauth2::UserToken;

#[derive(Debug, Serialize)]
struct SendChatMessageBody<'a> {
    broadcaster_id: &'a str,
    sender_id: &'a str,
    message: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_parent_message_id: Option<&'a str>,
}

#[derive(Debug, Deserialize)]
struct SendChatMessageResponse {
    data: Vec<SendChatMessageResult>,
}

#[derive(Debug, Deserialize)]
pub struct SendChatMessageResult {
    pub is_sent: bool,
    pub drop_reason: Option<DropReason>,
}

#[derive(Debug, Deserialize)]
pub struct DropReason{
    pub code: String,
    pub message: String,
}


pub async fn send_chat_message(
    broadcaster_id: &str,
    sender_id: &str,
    message: &str,
    reply_parent_message_id: Option<&str>,
    token: &UserToken,
) -> Result<SendChatMessageResult, Box<dyn std::error::Error>> {
    let client_id = env::var("TWITCH_CLIENT_ID")
        .expect("TWITCH_CLIENT_ID must be set");

    let body = SendChatMessageBody {
        broadcaster_id,
        sender_id,
        message,
        reply_parent_message_id,
    };

    let response = reqwest::Client::new()
        .post("https://api.twitch.tv/helix/chat/messages")
        .header("Client-Id", client_id)
        .header(
            "Authorization",
            format!("Bearer {}", token.access_token.secret()),
        )
        .json(&body)
        .send()
        .await?
        .error_for_status()?
        .json::<SendChatMessageResponse>()
        .await?;

    let result = response
        .data
        .into_iter()
        .next()
        .ok_or("Empty response")?;

    Ok(result)
}

