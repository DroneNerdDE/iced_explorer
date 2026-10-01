use std::{env, fs::read, path::PathBuf};
use crate::core::{read_directory, FileEntry};

// Represents the data our application holds at any given time.
pub struct FileExplorer {
    pub current_dir: PathBuf,
    pub files: Vec<FileEntry>,
    pub show_hidden: bool,
    pub history_back: Vec<PathBuf>,
    pub history_forward: Vec<PathBuf>,
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
            history_back: Vec::new(),
            history_forward: Vec::new(),
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
                state.history_back.push(state.current_dir.clone());
                state.history_forward.clear(); // Clear forward history if user selects new dir

                state.current_dir = path.clone();
                state.files = read_directory(&path, state.show_hidden);
            }
        },
        Message::Backward => {
            if let Some(prev_dir) = state.history_back.pop() {
                state.history_forward.push(state.current_dir.clone());

                state.current_dir = prev_dir.clone();
                state.files = read_directory(&prev_dir, state.show_hidden);
            }
        },
        Message::Forward => {
            if let Some(next_dir) = state.history_forward.pop() {
                state.history_back.push(state.current_dir.clone());

                state.current_dir = next_dir.clone();
                state.files = read_directory(&next_dir, state.show_hidden);
            }
        },
        Message::Home => {
            if let Some(user_dirs) = directories::UserDirs::new() {
                let home_dir = user_dirs.home_dir().to_path_buf();
                update(state, Message::Navigate(home_dir));
            } else {
                println!("Could not identify user's home directory!")
            }
        }
        Message::ToggleHidden(show) => {
            state.show_hidden = show;
            state.files = read_directory(&state.current_dir, state.show_hidden);
        }
    }
}