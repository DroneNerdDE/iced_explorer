mod core;
mod app;
mod view;

use app::{FileExplorer, update};
use view::view;

pub fn main() -> iced::Result {
    iced::application(FileExplorer::default, update, view)
        .title("Iced File Explorer")
        .run()
}