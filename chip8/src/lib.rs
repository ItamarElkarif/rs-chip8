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

pub use display::{Display, SCREEN_HEIGHT, SCREEN_WIDTH};
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
    delay_timer: u8,
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

impl<'a> Chip8 {
    pub fn advance(&mut self) {
        self.pc += 2;
    }

    pub fn run_frame(&'a mut self) -> Result<&'a Display, Box<dyn Error>> {
        self.display.should_redrew = false;
        let mut remaining = FRAME_DURATION;

        while remaining > Duration::ZERO {
            let inst = instruction::read(self)?;

            let took = execute(self, inst)?;

            remaining = remaining.saturating_sub(took);
        }

        self.delay_timer = self.delay_timer.saturating_sub(1);
        //TODO: beep if positive
        self.sound_timer = self.sound_timer.saturating_sub(1);

        Ok(&self.display)
    }
}
