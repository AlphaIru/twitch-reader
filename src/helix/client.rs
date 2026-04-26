//! helix/client.rs

use twitch_api::helix::HelixClient;

pub type TwitchHelix = HelixClient<'static, reqwest::Client>;


pub fn make_helix_client() -> TwitchHelix {
    HelixClient::default()
}
