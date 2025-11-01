use crossterm::terminal;
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    println!("=== Raw Input Test ===");
    println!("Press keys to see raw bytes from stdin.");
    println!("This bypasses crossterm's event parsing.");
    println!("Press Ctrl+D to exit.\n");
    
    // Enable raw mode
    terminal::enable_raw_mode()?;
    
    // Read loop
    let mut stdin = io::stdin();
    let mut buf = [0u8; 1];
    
    loop {
        match stdin.read(&mut buf) {
            Ok(0) => break,  // EOF
            Ok(_) => {
                let byte = buf[0];
                
                // Exit on Ctrl+D (0x04) or Ctrl+C (0x03)
                if byte == 0x04 || byte == 0x03 {
                    print!("\r\nReceived exit signal, exiting...");
                    io::stdout().flush()?;
                    break;
                }
                
                // Print byte info
                print!("\r\nByte: 0x{:02x} ({:3}) ", byte, byte);
                
                // Show printable representation
                if byte < 32 {
                    // Control character
                    let ctrl_char = if byte == 0 {
                        '@'
                    } else {
                        (byte + 64) as char
                    };
                    print!(" [Ctrl+{}]", ctrl_char);
                    
                    // Special cases
                    if byte == 0x1d {
                        print!(" ← THIS SHOULD BE Ctrl+]");
                    }
                } else if byte == 127 {
                    print!(" [DEL/Backspace]");
                } else if byte >= 32 && byte < 127 {
                    print!(" ['{}']", byte as char);
                } else {
                    print!(" [non-ASCII/escape sequence part]");
                }
                
                io::stdout().flush()?;
            }
            Err(e) => {
                print!("\r\nError: {}", e);
                io::stdout().flush()?;
                break;
            }
        }
    }
    
    terminal::disable_raw_mode()?;
    println!("\n\nDone!");
    Ok(())
}

