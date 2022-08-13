use crate::Output;
use crate::Editor;
use crossterm::{terminal};

pub struct CleanUp;

/*Return from Raw mode of terminal as soon as CleanUp gets out of Scope*/
impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Unable to disable raw mode");
        Output::clear_screen().expect("error");
    }
}

pub struct Terminal;


impl Terminal {
    pub fn init()-> crossterm::Result<()> {
        let _clean_up = CleanUp;
        terminal::enable_raw_mode()?;
        let mut editor = Editor::new();                 /*Call to Text Editor*/
        while editor.run()? {}
        Ok(())
    }                   /*CleanUp gets out of scope*/
}