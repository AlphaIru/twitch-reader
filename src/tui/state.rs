//! AlphaIru
//! Twitch Reader
//! 
//! tui/state.rs
//!
//! This is the file that handles TUI
//! states.
//!     

use crate::tui::BroadcasterProfile;
use crate::twitch::types::ChatPayload;


pub enum InputMode {
    Normal,
    Insert,
    Command,
}


pub struct AppState {
    pub input_text: String,
    pub mode: InputMode,
    pub logs: Vec<ChatPayload>,

    pub show_help: bool,
    pub show_details: bool,
    pub selected_index: usize,

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
            show_details: false,
            selected_index: 0,

            my_profile: BroadcasterProfile {
                id: "0".to_string(),
                login: "you".to_string(),
            },
        }
    }

    pub fn push_log(&mut self, log: ChatPayload) {
        self.logs.push(log);
        while self.logs.len() > 250 {
            self.logs.remove(0);
        }

        if !self.logs.is_empty()
            && self.selected_index >= self.logs.len() {
            self.selected_index = self.logs.len() - 1;
        }
    }
}

