use crossterm::{terminal};

mod editor;
mod output;
mod cursor_controller;
mod content_editor;
mod prompt;
mod reader;
mod row;
mod status_message;

use editor::Editor;
pub use output::Output;
pub use cursor_controller::CursorController;
pub use content_editor::EditorContents;
pub use reader::Reader;
pub use row::{Row, EditorRows};
pub use status_message::StatusMessage;
struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Unable to disable raw mode");
        Output::clear_screen().expect("error");
    }
}

pub fn main() -> crossterm::Result<()> {
    let _clean_up = CleanUp;
    terminal::enable_raw_mode()?;
    let mut editor = Editor::new();
    while editor.run()? {}
    Ok(())
}