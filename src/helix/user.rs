//! helix/user.rs

use twitch_api::twitch_oauth2::UserToken;

use crate::helix::client::TwitchHelix;

#[derive(Debug, Clone)]
pub struct BroadcasterProfile {
    pub id: String,
    pub login: String,
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

    Ok(BroadcasterProfile {
        id: user.id.to_string(),
        login: user.login.to_string(),
    })
}

