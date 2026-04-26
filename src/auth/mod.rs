//! AlphaIru
//! Twitch Reader
//!
//! auth/mod.rs
//! 
//! This is the module for twitch authentication,
//! and this one handles the authentication process
//!


pub mod oauth;
pub mod refresh;
pub mod store;


#[derive(Clone, Debug)]
pub struct AuthData {
    pub access_token: String,
}


pub async fn authenticate() -> Result<AuthData, Box<dyn std::error::Error>> {
    if let Some(saved_refresh_token) = store::load_token() {
        match refresh::refresh_token(saved_refresh_token).await {
            Ok((access_token, refresh_token)) => {
                if !refresh_token.is_empty() {
                    store::save_token(
                        Some(&access_token),
                        Some(&refresh_token),
                    )?;                    
                }                

                return Ok(AuthData {
                    access_token,
                });
            }      
            Err(e) => {
                eprintln!("Failed to refresh token: {}", e);
                eprintln!("Trying to get a new token...");
            }
        }
    }

    let (access_token, refresh_token) = oauth::get_token().await?;
    
    if !refresh_token.is_empty() {
        store::save_token(
            Some(&access_token), 
            Some(&refresh_token),
        )?;
    }

    Ok(AuthData {
        access_token,
    })

}
