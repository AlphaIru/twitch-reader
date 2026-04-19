//! AlphaIru
//! Twitch Reader
//! 
//! tui/state.rs
//!
//! This is the file that handles TUI
//! states.
//!     

use crate::tui::BroadcasterProfile;


pub enum InputMode {
    Normal,
    Insert,
    Command,
}


pub struct AppState {
    pub input_text: String,
    pub mode: InputMode,
    pub logs: Vec<String>,

    pub show_help: bool,

    pub scroll_offset: u16,

    pub my_profile: BroadcasterProfile,
}


impl AppState {
    pub fn new() -> Self {
        Self {
            input_text: String::new(),
            mode: InputMode::Normal,
            
            logs: Vec::new(),
            scroll_offset: 0,

            show_help: false,
        
            my_profile: BroadcasterProfile {
                id: "0".to_string(),
                login: "you".to_string(),
                display_name: "You".to_string(),
                color: "#FFFFFF".to_string(),
            },
        }
    }

    pub fn push_log(&mut self, log: String) {
        self.logs.push(log);
        if self.logs.len() > 250 {
            self.logs.remove(0);
        }
    }
}

