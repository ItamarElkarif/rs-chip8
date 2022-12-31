#![allow(dead_code)]
use crate::display::{Display, SCREEN_HEIGHT, SCREEN_WIDTH};
pub use display::{DisplayData, UI};
use std::{error::Error, fs::File, io::Read};

pub const MEM_SIZE: usize = 4 * 1024;
pub const ROM_START: usize = 0x200;

mod instruction;
use instruction::Instruction;
mod stack;

mod display {
    pub const SCREEN_WIDTH: usize = 64;
    pub const SCREEN_HEIGHT: usize = 32;
    pub type DisplayData = [[bool; SCREEN_HEIGHT]; SCREEN_WIDTH];
    pub trait UI {
        fn update(&mut self, display: &DisplayData);
    }

    pub struct Display {
        data: DisplayData,
        updated: bool,
    }

    impl Display {
        pub fn mut_data_to_update(&mut self) -> &mut [[bool; SCREEN_HEIGHT]; SCREEN_WIDTH] {
            self.updated = true;
            &mut self.data
        }

        pub fn data(&self) -> &[[bool; SCREEN_HEIGHT]; SCREEN_WIDTH] {
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
                data: [[false; SCREEN_HEIGHT]; SCREEN_WIDTH],
                updated: false,
            }
        }
    }
}

pub struct Chip8<'a> {
    memory: [u8; MEM_SIZE],
    display: Display, // TODO: Maybe make a struct with api since it is a 2dim array actually
    pc: u16,
    ip: u16,
    stack: stack::Stack,
    delay_timer: u8, // TODO: Maybe atomic? need to decrement it in another thread
    sound_timer: u8,
    keypads: u16, // TODO: use bitflags
    registers: [u8; 0x10],
    ui: &'a mut (dyn UI + 'a),
}

impl<'a> Chip8<'a> {
    pub fn new(ui: &'a mut dyn UI) -> Self {
        Self {
            memory: [0; MEM_SIZE],
            display: Display::default(),
            pc: ROM_START as u16,
            ip: Default::default(),
            stack: Default::default(),
            delay_timer: Default::default(),
            sound_timer: Default::default(),
            keypads: Default::default(),
            registers: Default::default(),
            ui,
        }
    }
}

impl Chip8<'_> {
    fn execute_instruction(&mut self, instruction: Instruction) -> Result<(), Box<dyn Error>> {
        // TODO: guard from invalid jump out of mem or getting incapable register, replace with array.get_mut
        dbg!(&instruction);
        dbg!(self.pc);
        match instruction {
            Instruction::CLS => {
                *self.display.mut_data_to_update() = [[false; SCREEN_HEIGHT]; SCREEN_WIDTH];
            }
            Instruction::RET => {
                self.pc = self.stack.pop().ok_or("Can't return, the stack is empty")?;
            }
            Instruction::SysJump(_) => unreachable!(),
            Instruction::JP(addr) => {
                self.pc = addr;
            }
            Instruction::LDReadRegisters(_) => {}
            Instruction::LDStoreRegisters(_) => todo!(),
            Instruction::LDStoreBCD(_) => todo!(),
            Instruction::LDSetISprite(_) => todo!(),
            Instruction::ADDI(_) => todo!(),
            Instruction::LDSetST(_) => todo!(),
            Instruction::LDSetDT(_) => todo!(),
            Instruction::LDKeyPress(_) => todo!(),
            Instruction::LDGetDT(_) => todo!(),
            Instruction::SKNP(_) => todo!(),
            Instruction::SKP(_) => todo!(),
            Instruction::DRW(vx, vy, n) => {
                // TODO: Fix function, doesn't seems to work
                let n = n & 15;
                let sprite = &self.memory[(self.ip as usize..(self.ip + n as u16) as usize)];
                for (y, p) in sprite.iter().enumerate() {
                    let c = vy as usize + y % SCREEN_HEIGHT;
                    for x in 0..8 {
                        let r = (vx + x) as usize % SCREEN_WIDTH;
                        let new_pixel = (p & (0b1 >> (7 - x))) != 0;

                        // If the xor going to erase the pixel (1^1), turn on the VF
                        if new_pixel & self.display.data()[r][c] {
                            self.registers[0xF] = 1;
                        }
                        self.display.mut_data_to_update()[r][c] ^= new_pixel;
                    }
                }
            }
            Instruction::RND(_, _) => todo!(),
            Instruction::LDSetIAddr(addr) => self.ip = addr,
            Instruction::V0JP(addr) => {
                self.pc = addr + self.registers[0] as u16;
            }
            Instruction::SHL(_, _) => todo!(),
            Instruction::SUBN(_, _) => todo!(),
            Instruction::SHR(_, _) => todo!(),
            Instruction::SUB(_, _) => todo!(),
            Instruction::XOR(_, _) => todo!(),
            Instruction::AND(_, _) => todo!(),
            Instruction::OR(_, _) => todo!(),
            Instruction::LDSetNibbles(_, _) => todo!(),
            // TODO: SHould check for overflow etc... low level stuff in all instructions
            Instruction::ADD(reg, val) => {
                self.registers[reg as usize] += val;
            }
            Instruction::LD(reg, val) => {
                self.registers[reg as usize] = val;
            }
            Instruction::SENibble(_, _) => todo!(),
            Instruction::SNE(_, _) => todo!(),
            Instruction::SEByte(reg, val) => {
                if self.registers[reg as usize] == val {
                    self.pc += 1;
                }
            }
            Instruction::CALL(addr) => {
                self.stack.push(self.pc)?;
                self.pc = addr;
            }
        };
        Ok(())
    }
}

// TODO: replace with frame iterators? how to handle input
pub fn run_file<'a>(
    emulator: &'a mut Chip8,
    file: &'a std::path::Path,
) -> Result<(), Box<dyn Error>> {
    load_rom(file, emulator)?;
    // TODO: start timers (delay and sound) - wrap chip in arc or check how much time passed since update
    while emulator.pc < emulator.memory.len() as u16 {
        let inst = read_instraction(emulator)?;
        // dbg!((emulator.pc, &inst));
        emulator.execute_instruction(inst)?;

        if emulator.display.updated() {
            emulator.ui.update(&emulator.display.data());
            emulator.display.reset_updated();
        }
    }
    Ok(())
}

fn load_rom(file: &std::path::Path, emulator: &mut Chip8) -> Result<(), Box<dyn Error>> {
    let mut rom = Vec::with_capacity(MEM_SIZE);
    let len = File::open(file)?.read_to_end(&mut rom)?;
    if emulator.memory.len() < ROM_START + len {
        return Err("The rom is too big!".into());
    }
    emulator.memory[ROM_START..ROM_START + len].copy_from_slice(&rom);
    Ok(())
}

fn read_instraction(emulator: &mut Chip8) -> Result<Instruction, Box<dyn Error>> {
    // TODO: consider making the mem just u16 since I split it in the try_from_opcode, might ruin
    // sprites
    let opcode: u16 = emulator.memory[emulator.pc as usize + 1] as u16
        | ((emulator.memory[emulator.pc as usize] as u16) << 8);
    emulator.pc += 2;
    Instruction::try_from(opcode)
}
