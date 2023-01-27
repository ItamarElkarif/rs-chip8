#![allow(dead_code)]
use std::{
    cmp::max,
    error::Error,
    time::{Duration, Instant},
};

pub const MEM_SIZE: usize = 4 * 1024;
pub const ROM_START: usize = 0x200;
pub const FRAME_DURATION: Duration = Duration::from_millis(17);

mod display;
mod instruction;
mod resources;
use instruction::{execute_instruction, Instruction};
mod stack;

use display::Display;
pub use display::{DisplayData, SCREEN_HEIGHT, SCREEN_WIDTH, UI};
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
    registers: [u8; 0x10],
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

    pub fn updated(&mut self) {
        self.display.reset_updated()
    }

    pub fn updated_display(&self) -> bool {
        self.display.updated()
    }

    pub fn advance(&mut self) {
        self.pc += 2;
    }
    // TODO: should return the display, and while dropped Frame reset the display
    pub fn run_frame(&mut self) -> Result<(), Box<dyn Error>> {
        // TODO: timers (delay and sound) - implement it better! Maybe with https://jackson-s.me/2019/07/13/Chip-8-Instruction-Scheduling-and-Frequency.html
        let start_iter = Instant::now();
        while (Instant::now() - start_iter) < FRAME_DURATION {
            let inst = read_instraction(self)?;
            execute_instruction(self, inst)?;

            update_timer(&mut self.delay_timer, start_iter);

            //TODO: beep
            update_timer(&mut self.sound_timer, start_iter);
        }
        Ok(())
    }
}

fn update_timer(timer: &mut u8, start_iter: Instant) {
    let delta_time = max((Instant::now() - start_iter).as_millis() * 1000 / 60, 0) as u8;
    if *timer > delta_time {
        *timer -= delta_time;
    } else {
        *timer = 0;
    }
}

fn read_instraction(emulator: &mut Chip8) -> Result<Instruction, Box<dyn Error>> {
    let opcode: (u8, u8) = (
        emulator.memory[emulator.pc as usize],
        emulator.memory[emulator.pc as usize + 1],
    );
    emulator.advance();
    Instruction::try_from(opcode)
}
