//! AlphaIru
//! Twitch Reader
//!
//! auth/refresh.rs
//! 
//! This is the module for twitch authentication,
//! and this one handles the refresh process
//!


use std::env;

use twitch_api::twitch_oauth2::{
    ClientId,
    ClientSecret,
    RefreshToken,
    UserToken,
};

pub async fn refresh_token(
    refresh_token_str: String,
) -> Result<(String, String), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()?;

    let client_id = env::var("TWITCH_CLIENT_ID")
        .expect("TWITCH_CLIENT_ID must be set!");
    let client_secret = env::var("TWITCH_CLIENT_SECRET")
        .expect("TWITCH_CLIENT_SECRET must be set!");

    let token = UserToken::from_refresh_token(
        &client,
        RefreshToken::new(refresh_token_str),
        ClientId::from(client_id),
        ClientSecret::from(client_secret),
    )
    .await?;

    let access_token = token.access_token.secret().to_string();
    let new_refresh_token = token
        .refresh_token
        .as_ref()
        .map(|r| r.secret().to_string())
        .unwrap_or_default();

    Ok((access_token, new_refresh_token))
}


