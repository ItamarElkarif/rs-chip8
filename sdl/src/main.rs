mod resources;
use chip8::{Chip8, MEM_SIZE};
use std::fs::File;
use std::io::Read;
use std::{error::Error, path::Path};

struct ConsoleUI;
impl chip8::UI for ConsoleUI {
    fn update(&mut self, display: &chip8::DisplayData) {
        print!("{esc}c", esc = 27 as char);
        for row in display.chunks(chip8::SCREEN_WIDTH) {
            println!(
                "{}",
                row.iter()
                    .map(|&p| if p { "█" } else { " " })
                    .collect::<Vec<&str>>()
                    .join("")
            )
        }
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let mut sdl = ConsoleUI;
    let mut chip = Chip8::new(&mut sdl);
    let mut rom = Vec::with_capacity(MEM_SIZE);
    File::open(Path::new("D:/Code/rust/rs-chip8/IBM Logo.ch8"))?.read_to_end(&mut rom)?;
    chip8::run_file(&mut chip, &rom)?;
    Ok(())
}
