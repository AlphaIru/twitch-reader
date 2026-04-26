//! helix/types.rs

#[derive(Debug, Clone)]
pub struct CommandConfig {
    pub broadcaster_login: String,
    pub broadcaster_id: String,  
    pub moderator_id: String,
}
