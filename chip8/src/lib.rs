#![allow(dead_code)]
use crate::display::Display;
pub use crate::display::{SCREEN_HEIGHT, SCREEN_WIDTH};
pub use display::{DisplayData, UI};
use resources::SPRITE_ADDR;
use stack::Stack;
use std::{cmp::max, error::Error, time::Instant};

pub const MEM_SIZE: usize = 4 * 1024;
pub const ROM_START: usize = 0x200;

mod instruction;
mod resources;
use instruction::{execute_instruction, Instruction};
mod stack;

mod display {
    pub const SCREEN_WIDTH: usize = 64;
    pub const SCREEN_HEIGHT: usize = 32;
    pub type DisplayData = [bool; SCREEN_WIDTH * SCREEN_HEIGHT];
    pub trait UI {
        fn update(&mut self, display: &DisplayData);
        fn beep(&self);
    }

    pub struct Display {
        data: DisplayData,
        updated: bool,
    }

    impl Display {
        pub fn mut_data_to_update(&mut self) -> &mut [bool; SCREEN_WIDTH * SCREEN_HEIGHT] {
            self.updated = true;
            &mut self.data
        }

        pub fn data(&self) -> &[bool; SCREEN_WIDTH * SCREEN_HEIGHT] {
            &self.data
        }

        pub fn updated(&self) -> bool {
            self.updated
        }

        pub fn reset_updated(&mut self) {
            self.updated = false;
        }
    }

    impl Default for Display {
        fn default() -> Display {
            Display {
                data: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
                updated: false,
            }
        }
    }
}

pub struct Chip8<'a> {
    memory: [u8; MEM_SIZE],
    display: Display,
    pc: u16,
    i: u16,
    stack: Stack,
    delay_timer: u8, // TODO: Maybe atomic? need to decrement it in another thread
    sound_timer: u8,
    keypads: u16, // TODO: use bitflags
    registers: [u8; 0x10],
    ui: &'a mut (dyn UI + 'a),
}

impl<'a> Chip8<'a> {
    pub fn new(ui: &'a mut dyn UI) -> Self {
        let mut mem = [0; MEM_SIZE];
        mem[0..0x10 * 5].copy_from_slice(&SPRITE_ADDR);

        Self {
            memory: mem,
            display: Display::default(),
            pc: ROM_START as u16,
            i: Default::default(),
            stack: Default::default(),
            delay_timer: Default::default(),
            sound_timer: Default::default(),
            keypads: Default::default(),
            registers: Default::default(),
            ui,
        }
    }
}

impl<'a> Chip8<'a> {
    fn load_rom(&mut self, input: &[u8]) -> Result<(), Box<dyn Error>> {
        if self.memory.len() < ROM_START + input.len() {
            return Err("The rom is too small!".into());
        }
        self.memory[ROM_START..ROM_START + input.len()].copy_from_slice(input);
        Ok(())
    }
}

// TODO: replace with frame iterators? how to handle input
pub fn run_file<'a>(emulator: &'a mut Chip8, file: &[u8]) -> Result<(), Box<dyn Error>> {
    emulator.load_rom(file)?;
    // TODO: start timers (delay and sound) - implement it better! Maybe with https://jackson-s.me/2019/07/13/Chip-8-Instruction-Scheduling-and-Frequency.html
    while emulator.pc < emulator.memory.len() as u16 {
        let start_iter = Instant::now();

        let inst = read_instraction(emulator)?;
        execute_instruction(emulator, inst)?;

        if emulator.delay_timer > 0 {
            update_timer(&mut emulator.delay_timer, start_iter);
        }

        if emulator.sound_timer > 0 {
            emulator.ui.beep();
            update_timer(&mut emulator.sound_timer, start_iter);
        }

        if emulator.display.updated() {
            emulator.ui.update(emulator.display.data());
            emulator.display.reset_updated();
        }
    }
    Ok(())
}

fn update_timer(timer: &mut u8, start_iter: Instant) {
    *timer -= max((Instant::now() - start_iter).as_millis() * 1000 / 60, 0) as u8;
}

fn read_instraction(emulator: &mut Chip8) -> Result<Instruction, Box<dyn Error>> {
    let opcode: u16 = emulator.memory[emulator.pc as usize + 1] as u16
        | ((emulator.memory[emulator.pc as usize] as u16) << 8);
    emulator.pc += 2;
    Instruction::try_from(opcode)
}
