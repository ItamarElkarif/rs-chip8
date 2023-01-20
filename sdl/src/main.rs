#![allow(dead_code)]
use chip8::{Chip8, MEM_SIZE, SCREEN_HEIGHT, SCREEN_WIDTH};

use egui::*;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::{error::Error, path::Path};

struct ConsoleUI();
impl ConsoleUI {
    fn update(&self, display: &chip8::DisplayData) {
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

    fn beep(&self) {
        println!("BEEPED");
    }
}

#[derive(Debug)]
struct ChipInput(u16);
impl From<&HashSet<Key>> for ChipInput {
    fn from(keys: &HashSet<Key>) -> Self {
        let mut keys_buffer: u16 = 0;
        for key in keys {
            keys_buffer |= match key {
                Key::Num1 => 1 << 0x1,
                Key::Num2 => 1 << 0x2,
                Key::Num3 => 1 << 0x3,
                Key::Num4 => 1 << 0xC,
                Key::Q => 1 << 0x4,
                Key::W => 1 << 0x5,
                Key::E => 1 << 0x6,
                Key::R => 1 << 0xD,
                Key::A => 1 << 0x7,
                Key::S => 1 << 0x8,
                Key::D => 1 << 0x9,
                Key::F => 1 << 0xE,
                Key::Z => 1 << 0xA,
                Key::X => 1 << 0x0,
                Key::C => 1 << 0xB,
                Key::V => 1 << 0xF,
                _ => 0,
            }
        }
        ChipInput(keys_buffer)
    }
}
struct App {
    chip: Chip8,
}
impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let keys_buffer: ChipInput = (&ctx.input().keys_down).into();

        self.chip.set_keypad(keys_buffer.0);
        self.chip.run_frame().unwrap();

        //TODO: Beep if needed
        egui::CentralPanel::default().show(ctx, |ui| {
            self.display_screen(ui);
        });

        if self.chip.updated_display() {
            self.chip.updated();
        }
    }
}

impl App {
    fn new(chip: Chip8) -> Self {
        Self { chip }
    }

    fn display_screen(&mut self, ui: &mut Ui) {
        let tile_width = ui.available_width() / (SCREEN_WIDTH - 1) as f32;
        let tile_height = ui.available_height() / (SCREEN_HEIGHT - 1) as f32;
        for row in 0..SCREEN_WIDTH {
            let x = tile_width * row as f32;
            for col in 0..SCREEN_HEIGHT {
                let y = tile_height * col as f32;
                ui.painter().rect(
                    Rect {
                        min: Pos2 { x, y },
                        max: Pos2 {
                            x: x + tile_width,
                            y: y + tile_height,
                        },
                    },
                    Rounding::none(),
                    if self.chip.display()[row + col * SCREEN_WIDTH] {
                        Color32::WHITE
                    } else {
                        Color32::BLACK
                    },
                    Stroke::NONE,
                );
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut rom = Vec::with_capacity(MEM_SIZE);
    File::open(Path::new("D:/Code/rust/rs-chip8/IBM Logo.ch8"))?.read_to_end(&mut rom)?;
    let chip = Chip8::new(&rom[..])?;
    eframe::run_native(
        "Chip8",
        eframe::NativeOptions::default(),
        Box::new(|_| Box::new(App::new(chip))),
    );
    Ok(())
}
