mod editor;
mod output;
mod cursor_controller;
mod content_editor;
mod prompt;
mod reader;
mod row;
mod status_message;
mod terminal;

use crossterm;
pub use editor::Editor;
pub use output::Output;
pub use cursor_controller::CursorController;
pub use content_editor::EditorContents;
pub use reader::Reader;
pub use row::{Row, EditorRows};
pub use status_message::StatusMessage;
use terminal::{Terminal, CleanUp};

pub fn main() -> crossterm::Result<()>{
    Terminal::init()
}