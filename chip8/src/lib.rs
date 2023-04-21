#![allow(dead_code)]
use std::{error::Error, time::Duration};

pub const MEM_SIZE: usize = 4 * 1024;
pub const ROM_START: usize = 0x200;
pub const FRAME_DURATION: Duration = Duration::from_micros(16666);

mod display;
mod instruction;
mod registers;
mod resources;
mod stack;

use display::Display;
pub use display::{DisplayData, SCREEN_HEIGHT, SCREEN_WIDTH, UI};
use instruction::execute;
use registers::Regs;
use resources::SPRITE_ADDR;
use stack::Stack;

pub struct Chip8 {
    memory: [u8; MEM_SIZE],
    display: Display,
    pc: u16,
    i: u16,
    stack: Stack,
    delay_timer: u8, // TODO: Maybe atomic? need to decrement it in another thread
    sound_timer: u8,
    keypad: u16, // TODO: use bitflags
    registers: Regs,
}

fn load_rom(mem: &mut [u8], input: &[u8]) -> Result<(), Box<dyn Error>> {
    if mem.len() < ROM_START + input.len() {
        return Err("The rom is too small!".into());
    }
    mem[ROM_START..ROM_START + input.len()].copy_from_slice(input);
    Ok(())
}

impl Chip8 {
    pub fn new<'a>(file: impl Into<&'a [u8]>) -> Result<Self, Box<dyn Error>> {
        let mut mem = [0; MEM_SIZE];
        mem[0..0x10 * 5].copy_from_slice(&SPRITE_ADDR);
        load_rom(&mut mem, file.into())?;

        Ok(Self {
            memory: mem,
            display: Display::default(),
            pc: ROM_START as u16,
            i: Default::default(),
            stack: Default::default(),
            delay_timer: Default::default(),
            sound_timer: Default::default(),
            keypad: Default::default(),
            registers: Default::default(),
        })
    }

    pub fn set_keypad(&mut self, new: u16) {
        self.keypad = new;
    }
}

impl Chip8 {
    pub fn display(&self) -> &DisplayData {
        self.display.data()
    }

    // TODO: make it automatic after using it in the exe
    pub fn reset_updated(&mut self) {
        self.display.reset_updated()
    }

    pub fn updated_display(&self) -> bool {
        self.display.updated()
    }

    pub fn advance(&mut self) {
        self.pc += 2;
    }

    // TODO: should return the display, and while dropped Frame reset the display
    // Make the return type an Result<Option<Display>, Box<dyn Error>> and NONE if not changed
    pub fn run_frame(&mut self) -> Result<(), Box<dyn Error>> {
        let mut remaining = FRAME_DURATION;
        while remaining > Duration::ZERO {
            self.delay_timer = self.delay_timer.saturating_sub(1);
            //TODO: beep
            self.sound_timer = self.sound_timer.saturating_sub(1);
            let inst = instruction::read(self)?;

            let took = execute(self, inst)?;

            remaining = remaining.saturating_sub(took);
            
        }
        Ok(())
    }
}
