use std::env;
use std::error::Error;

use reqwest::Client;

use crate::auth::config::Config;

pub async fn validate_token(token: &str) -> bool {
    let client = Client::new();
    let auth_val = format!("oauth:{}", "");

    client.get("https://id.twitch.tv/oauth2/validate")
        .header("Authorization", format!("OAuth {}", auth_val))
        .send()
        .await
        .map(|r| r.status().is_success())
        .unwrap_or(false)
}


pub async fn refresh_access_token(refresh_token: &str) -> Result<String, Box<dyn Error>>{
    let client_id = env::var("TWITCH_CLIENT_ID").expect("Error: .env file not found or TWITCH_CLIENT_ID must be set");
    let client_secret = env::var("TWITCH_CLIENT_SECRET").expect("Error: .env file not found or TWITCH_CLIENT_SECRET must be set");
    
    let client = Client::new();
    let params = [
        ("grant_type", "refresh_token"),
        ("refresh_token", refresh_token),
        ("client_id", &client_id),
        ("client_secret", &client_secret),
    ];

    let resp = client.post("https://id.twitch.tv/oauth2/token")
        .form(&params)
        .send()
        .await?
        .json::<serde_json::Value>().await?;

    Ok(Config {
        username: "".into(),
        token: format!("oauth:{}", resp["access_token"].as_str().unwrap()),
        refresh_token: Some(resp["refresh_token"].as_str().unwrap().to_string()),
    })
}
