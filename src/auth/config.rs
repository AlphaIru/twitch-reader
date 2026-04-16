use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub client_id: String,
    pub client_secret: String,
    pub refresh_token: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            client_id: "".into(),
            client_secret: "".into(),
            refresh_token: None,
        }
    }
}
