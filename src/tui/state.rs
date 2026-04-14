//! AlphaIru
//! Twitch Reader
//! 
//! tui/state.rs
//!
//! This is the file that handles TUI
//! states.
//!     


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

    pub my_name: String,
    pub my_color: String,
}


impl AppState {
    pub fn new() -> Self {
        Self {
            input_text: String::new(),
            mode: InputMode::Normal,
            logs: Vec::new(),
            show_help: false,
            
            my_name: "You".to_string(),
            my_color: "#FFFFFF".to_string(),
        }
    }

    pub fn push_log(&mut self, log: String) {
        self.logs.push(log);
        if self.logs.len() > 100 {
            self.logs.remove(0);
        }
    }
}

