#![allow(dead_code)]
mod console_ui;
mod egui;

use chip8::{Chip8, MEM_SIZE};

use std::{env::args, error::Error, fs::File, io::Read, path::Path};

// TODO: Make use of it? the main should have a UI and run it with the chip maybe
trait Ui {
    fn run(chip: Chip8);
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = args().nth(1).expect("Need a path for chip8 game");
    let mut rom = Vec::with_capacity(MEM_SIZE);

    File::open(Path::new(&file))?.read_to_end(&mut rom)?;
    let chip = Chip8::new(&rom[..])?;

    egui::App::run(chip);
    Ok(())
}
