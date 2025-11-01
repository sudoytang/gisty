use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::terminal;

fn main() -> std::io::Result<()> {
    terminal::enable_raw_mode()?;
    loop {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                println!("\r\nReceived: {:?}", key);
                if matches!(key.code, KeyCode::Char('c')) 
                    && key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) {
                    break;
                }
            }
        }
    }
    terminal::disable_raw_mode()?;
    Ok(())
}