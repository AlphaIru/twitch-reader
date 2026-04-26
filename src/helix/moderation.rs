//! helix/moderation.rs


use std::env;

use twitch_api::twitch_oauth2::UserToken;


pub async fn clear_chat(
    broadcaster_id: &str,
    moderator_id: &str,
    token: &UserToken,
) -> Result<(), Box<dyn std::error::Error>> {
    let client_id = env::var("TWITCH_CLIENT_ID")
        .expect("Error: .env file not found or TWITCH_CLIENT_ID must be set");

    let url = format!(
        "https://api.twitch.tv/helix/moderation/chat?broadcaster_id={}&moderator_id={}",
        broadcaster_id, moderator_id
    );

    let _ = reqwest::Client::new()
        .delete(url)
        .header("Client-Id", client_id)
        .header(
            "Authorization",
            format!("Bearer {}", token.access_token.secret())
        )
        .send()
        .await?
        .error_for_status()?;

    Ok(())
}
