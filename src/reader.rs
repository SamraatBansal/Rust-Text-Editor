use crossterm::event::*;
use crossterm::event;


pub struct Reader;

impl Reader {
    pub fn read_key(&self) -> crossterm::Result<KeyEvent> {
        loop {
                if let Event::Key(event) = event::read()? {
                    return Ok(event);
                }
        }
    }
}