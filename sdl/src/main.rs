mod resources;
use chip8::Chip8;
use std::{error::Error, path::Path};

struct SDL;
impl chip8::UI for SDL {
    fn update(&mut self, display: &chip8::DisplayData) {
        // print!("{esc}c", esc = 27 as char);
        for row in display {
            println!("{}", row.map(|p| if p { "█" } else { " " }).join(""))
        }
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let mut sdl = SDL;
    let mut chip = Chip8::new(&mut sdl);
    chip8::run_file(&mut chip, Path::new("D:/Code/rust/rs-chip8/IBM Logo.ch8"))?;
    Ok(())
}
