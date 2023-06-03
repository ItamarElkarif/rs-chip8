use std::{collections::HashSet, ops::Deref};

use chip8::{Chip8, Display, FRAME_DURATION, SCREEN_HEIGHT, SCREEN_WIDTH};
use egui::{epaint::RectShape, *};
pub struct App {
    chip: Chip8,
}

impl crate::Ui for App {
    fn run(chip: Chip8) {
        eframe::run_native(
            "Chip8",
            eframe::NativeOptions::default(),
            Box::new(|_| Box::new(App { chip })),
        )
        .unwrap();
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let keys = ctx.input(|i| ChipInput::from(&i.keys_down));

        self.chip.set_keypad(*keys);

        let (display, _beep) = self.chip.run_frame().unwrap();
        egui::CentralPanel::default().show(ctx, |ui| {
            App::display_screen(ui, display);
        });

        //TODO: Beep if needed

        std::thread::sleep(FRAME_DURATION);
        ctx.request_repaint();
    }
}

impl App {
    fn display_screen(ui: &mut Ui, display: &Display) {
        let tile_width = ui.available_width() / (SCREEN_WIDTH - 1) as f32;
        let tile_height = ui.available_height() / (SCREEN_HEIGHT - 1) as f32;
        for row in 0..SCREEN_WIDTH {
            let x = tile_width * row as f32;
            for col in 0..SCREEN_HEIGHT {
                let y = tile_height * col as f32;
                ui.painter().add(tile(
                    x,
                    y,
                    tile_width,
                    tile_height,
                    display[row + col * SCREEN_WIDTH],
                ));
            }
        }
    }
}

fn tile(x: f32, y: f32, width: f32, height: f32, active: bool) -> RectShape {
    RectShape {
        rect: Rect {
            min: Pos2 { x, y },
            max: Pos2 {
                x: x + width,
                y: y + height,
            },
        },
        rounding: Rounding::none(),
        fill: if active {
            Color32::WHITE
        } else {
            Color32::BLACK
        },
        stroke: Stroke::NONE,
    }
}

#[derive(Debug)]
struct ChipInput(u16);
impl Deref for ChipInput {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

macro_rules! keysAt {
    ($key:expr, $($variant:pat, $at:expr);+) => {
        match $key {
           $($variant => 1 << $at,)+
           _ => 0,
        }
    };
}

impl From<&HashSet<Key>> for ChipInput {
    fn from(keys: &HashSet<Key>) -> Self {
        let mut keys_buffer: u16 = 0;
        for key in keys {
            keys_buffer |= keysAt!(key, 
            Key::Num1, 0x1;
            Key::Num2, 0x2;
            Key::Num3, 0x3;
            Key::Num4, 0xC;
            Key::Q, 0x4;
            Key::W, 0x5;
            Key::E, 0x6;
            Key::R, 0xD;
            Key::A, 0x7;
            Key::S, 0x8;
            Key::D, 0x9;
            Key::F, 0xE;
            Key::Z, 0xA;
            Key::X, 0x0;
            Key::C, 0xB;
            Key::V, 0xF);
        }
        ChipInput(keys_buffer)
    }
}
