use std::path::PathBuf;
use crate::core::{read_directory, FileEntry};

// Represents the data our application holds at any given time.
pub struct FileExplorer {
    pub current_dir: PathBuf,
    pub files: Vec<FileEntry>,
    pub show_hidden: bool,
}

impl Default for FileExplorer {
    fn default() -> Self {
        // Start in the direectory where the app was launched
        let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let show_hidden = false; 
        Self {
            files: read_directory(&current_dir, show_hidden), 
            current_dir,
            show_hidden,
        }
    }
}

// Represents all possible user interactions.
#[derive(Debug, Clone)]
pub enum Message {
    Navigate(PathBuf),
    ToggleHidden(bool),
    Home, // Jump to home dir of user
    Backward, // Jump to previous path (history backward)
    Forward, // Jump to previous path (history forward)
}

// Reacts to messages and modifies the state.
pub fn update(state: &mut FileExplorer, message: Message) {
    match message {
        Message::Navigate(path) => {
            if path.is_dir() {
                state.current_dir = path.clone();
                state.files = read_directory(&path, state.show_hidden);
            }
        }
        Message::ToggleHidden(show) => {
            state.show_hidden = show;
            state.files = read_directory(&state.current_dir, state.show_hidden);
        },
        Message::Home | Message::Backward | Message::Forward => todo!()
    }
}