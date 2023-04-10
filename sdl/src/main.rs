#![allow(dead_code)]
mod console_ui;
mod egui;

use chip8::{Chip8, MEM_SIZE};

use std::{error::Error, fs::File, io::Read, path::Path};

trait Ui {
    fn run(chip: Chip8);
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut rom = Vec::with_capacity(MEM_SIZE);
    File::open(Path::new("D:/Code/rust/rs-chip8/INVADERS.ch8"))?.read_to_end(&mut rom)?;
    let chip = Chip8::new(&rom[..])?;

    egui::App::run(chip);
    Ok(())
}
