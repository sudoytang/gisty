use std::io::Read;
use crossterm::terminal;

fn main() -> std::io::Result<()> {
    terminal::enable_raw_mode()?;
    let mut buf = [0u8; 1];
    std::io::stdin().read_exact(&mut buf)?;
    println!("Raw byte: 0x{:02x}", buf[0]);  // Prints: 0x1d
    terminal::disable_raw_mode()?;
    Ok(())
}