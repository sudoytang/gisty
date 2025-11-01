use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::terminal;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    println!("=== Keyboard Event Tester ===");
    println!("Press any key to see how it's interpreted by crossterm.");
    println!("Press Ctrl+C or Ctrl+D to exit.\n");
    
    // Enable raw mode
    terminal::enable_raw_mode()?;
    
    loop {
        if let Event::Key(key_event) = event::read()? {
            if key_event.kind == KeyEventKind::Press {
                print_key_event(&key_event);
                
                // Exit on Ctrl+C or Ctrl+D
                if key_event.modifiers.contains(KeyModifiers::CONTROL) {
                    if matches!(key_event.code, KeyCode::Char('c') | KeyCode::Char('d')) {
                        break;
                    }
                }
            }
        }
    }
    
    terminal::disable_raw_mode()?;
    println!("\nExiting...");
    Ok(())
}

fn print_key_event(key_event: &KeyEvent) {
    let KeyEvent { code, modifiers, .. } = key_event;
    
    print!("\r\n"); // New line in raw mode
    print!("Key: {:?}", code);
    
    // Show character and hex value if it's a char
    if let KeyCode::Char(c) = code {
        print!(" (char='{}', hex=0x{:02x}, dec={})", c, *c as u8, *c as u8);
    }
    
    print!("  |  Modifiers: ");
    if modifiers.is_empty() {
        print!("NONE");
    } else {
        if modifiers.contains(KeyModifiers::CONTROL) {
            print!("CTRL ");
        }
        if modifiers.contains(KeyModifiers::SHIFT) {
            print!("SHIFT ");
        }
        if modifiers.contains(KeyModifiers::ALT) {
            print!("ALT ");
        }
        if modifiers.contains(KeyModifiers::SUPER) {
            print!("SUPER ");
        }
        if modifiers.contains(KeyModifiers::HYPER) {
            print!("HYPER ");
        }
        if modifiers.contains(KeyModifiers::META) {
            print!("META ");
        }
    }
    
    // Show what byte would be sent
    let bytes = key_to_bytes(*key_event);
    if !bytes.is_empty() {
        print!("  |  Bytes: [");
        for (i, byte) in bytes.iter().enumerate() {
            if i > 0 { print!(", "); }
            print!("0x{:02x}", byte);
        }
        print!("]");
    }
    
    io::stdout().flush().ok();
}

// Simplified version of key_to_bytes for testing
fn key_to_bytes(key_event: KeyEvent) -> Vec<u8> {
    let KeyEvent { code, modifiers, .. } = key_event;
    
    let ctrl = modifiers.contains(KeyModifiers::CONTROL);
    let alt = modifiers.contains(KeyModifiers::ALT);
    
    match code {
        KeyCode::Char(c) => {
            if ctrl {
                if c.is_ascii_lowercase() || c.is_ascii_uppercase() {
                    let byte = (c.to_ascii_lowercase() as u8) - b'a' + 1;
                    vec![byte]
                } else if c == '@' {
                    vec![0x00]
                } else if c == '[' {
                    vec![0x1b]
                } else if c == '\\' {
                    vec![0x1c]
                } else if c == ']' {
                    vec![0x1d]
                } else if c == '^' {
                    vec![0x1e]
                } else if c == '_' {
                    vec![0x1f]
                } else {
                    c.to_string().into_bytes()
                }
            } else if alt {
                vec![0x1b, c as u8]
            } else {
                c.to_string().into_bytes()
            }
        }
        KeyCode::Enter => vec![b'\r'],
        KeyCode::Tab => vec![b'\t'],
        KeyCode::Backspace => vec![0x7f],
        KeyCode::Esc => vec![0x1b],
        _ => vec![],
    }
}

