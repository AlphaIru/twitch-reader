//! helix/user.rs

use twitch_api::twitch_oauth2::UserToken;

use crate::helix::client::TwitchHelix;


pub async fn get_user(
    client: &TwitchHelix,
    token: &UserToken,
    login: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let user = client
        .get_user_from_login(login, token)
        .await?
        .ok_or("User not found!")?;

    Ok(user.id.to_string())
}
