#[macro_export]
macro_rules! prompt {
    ($output:expr,$($args:tt)*) => {{
        let output:&mut Output = $output;
        let mut input = String::with_capacity(32);
        loop {
            output.status_message.set_message(format!($($args)*, input));
            output.refresh_screen()?;
            match Reader.read_key()? {
                KeyEvent {
                    code:KeyCode::Enter,
                    modifiers:KeyModifiers::NONE,
                    ..
                } => {
                    if !input.is_empty() {
                        output.status_message.set_message(String::new());
                        break;
                    }
                }
                KeyEvent {
                    code: KeyCode::Esc, ..
                } => {
                    output.status_message.set_message(String::new());
                    input.clear();
                    break;
                }
                KeyEvent {
                    code: KeyCode::Backspace | KeyCode::Delete,
                    modifiers: KeyModifiers::NONE,
                    ..
                } => { input.pop(); }
                KeyEvent {
                    code: code @ (KeyCode::Char(..) | KeyCode::Tab),
                    modifiers: KeyModifiers::NONE | KeyModifiers::SHIFT,
                    ..
                } => input.push(match code {
                        KeyCode::Tab => '\t',
                        KeyCode::Char(ch) => ch,
                        _ => unreachable!(),
                    }),
                _=> {}
            }
        }
        if input.is_empty() { None } else { Some (input) }
    }};
}