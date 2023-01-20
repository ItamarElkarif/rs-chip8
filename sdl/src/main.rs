#![allow(dead_code)]
use chip8::{Chip8, MEM_SIZE, SCREEN_HEIGHT, SCREEN_WIDTH};

use egui::*;
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

struct App(Chip8);
impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        //TODO: Handle Keys via ctx.input()
        self.0.run_frame().unwrap();

        if self.0.updated_display() {
            self.0.updated();
        }

        //TODO: Beep if needed
        egui::CentralPanel::default().show(ctx, |ui| {
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
                        if self.0.display()[row + col * SCREEN_WIDTH] {
                            Color32::WHITE
                        } else {
                            Color32::BLACK
                        },
                        Stroke::NONE,
                    );
                }
            }
        });
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut rom = Vec::with_capacity(MEM_SIZE);
    File::open(Path::new("D:/Code/rust/rs-chip8/IBM Logo.ch8"))?.read_to_end(&mut rom)?;
    let chip = Chip8::new(&rom[..])?;
    eframe::run_native(
        "Chip8",
        eframe::NativeOptions::default(),
        Box::new(|_| Box::new(App(chip))),
    );
    Ok(())
}
