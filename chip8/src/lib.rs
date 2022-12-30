#![allow(dead_code)]
use std::{
    fs::File,
    io::{BufReader, Read},
    thread,
    time::{Duration, Instant},
};

pub const MEM_SIZE: usize = 4 * 1024;
pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

mod instruction;
use instruction::Instruction;
mod stack;

pub struct Display {
    data: [[bool; SCREEN_HEIGHT]; SCREEN_WIDTH],
    updated: bool,
}

impl Display {
    fn mut_data_to_update(&mut self) -> &mut [[bool; SCREEN_HEIGHT]; SCREEN_WIDTH] {
        self.updated = true;
        &mut self.data
    }

    fn data(&self) -> &[[bool; SCREEN_HEIGHT]; SCREEN_WIDTH] {
        &self.data
    }

    fn reset_update(&mut self) {
        self.updated = false;
    }

    fn default() -> Display {
        Display {
            data: [[false; SCREEN_HEIGHT]; SCREEN_WIDTH],
            updated: false,
        }
    }
}
pub struct Chip8<'a> {
    memory: [u8; MEM_SIZE],
    display: Display, // TODO: Maybe make a struct with api since it is a 2dim array actually
    need_redraw: bool,
    pc: u16,
    ip: u16,
    stack: stack::Stack,
    delay_timer: u8, // TODO: Maybe atomic? need to decrement it in another thread
    sound_timer: u8,
    keypads: u16, // TODO: use bitflags
    registers: [u8; 0x10],
    ui: &'a mut (dyn UI + 'a),
}

pub trait UI {
    fn update(&mut self, display: &Display);
}

impl<'a> Chip8<'a> {
    pub fn new(ui: &'a mut dyn UI) -> Self {
        Self {
            memory: [0; MEM_SIZE],
            display: Display::default(),
            need_redraw: false,
            pc: Default::default(),
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
    fn execute_instruction(&mut self, instruction: Instruction) -> Result<(), &'static str> {
        self.need_redraw = false;
        // TODO: guard from invalid jump out of mem or getting incapable register, replace with array.get_mut
        match instruction {
            Instruction::CLS => {
                *self.display.mut_data_to_update() = [[false; SCREEN_HEIGHT]; SCREEN_WIDTH];
                self.need_redraw = true;
            }
            Instruction::RET => {
                self.pc = self.stack.top().unwrap();
                self.stack.pop().ok_or("Can't return, the stack is empty")?;
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
            Instruction::DRW(regx, regy, n) => {
                // Makes sure n is smaller than 15, the sprite max size
                let n = n & 15;
                let sprite = &self.memory[(self.ip as usize..(self.ip + n as u16) as usize)];
                let vx = self.registers[regx as usize];
                let vy = self.registers[regy as usize];
                for (y, p) in sprite.iter().enumerate() {
                    let c = vy as usize + y % SCREEN_HEIGHT;
                    for x in 0..8 {
                        let r = (vx + x) as usize % SCREEN_WIDTH;
                        let new_pixel = (p & (0b1 >> (8 - x))) != 0;

                        // If the xor going to erase the pixel (1^1), turn on the VF
                        if new_pixel & self.display.data()[r][c] {
                            self.registers[0xF] = 1;
                        }
                        self.display.mut_data_to_update()[r][c] ^= new_pixel;
                    }
                    self.need_redraw = true;
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
pub fn run_file<'a>(emulator: &'a mut Chip8, file: &'a std::path::Path) -> Result<(), &'a str> {
    let mut file_content = BufReader::new(File::open(file).unwrap());
    // TODO: start timers (delay and sound) - wrap chip in arc or check how much time passed since update
    // parse file?
    let mut rdr = [0u8; 2];
    loop {
        let start_iter = Instant::now();

        file_content.read_exact(&mut rdr).unwrap();
        let opcode: u16 = rdr[0] as u16 | ((rdr[1] as u16) << 8);
        let inst = Instruction::from(opcode);

        emulator.pc += rdr.len() as u16;
        emulator.execute_instruction(inst)?;

        if emulator.display.updated {
            emulator.ui.update(&emulator.display);
            emulator.display.updated = false;
        }
        // Delay iteration of the loop, use 500HZ or https://jackson-s.me/2019/07/13/Chip-8-Instruction-Scheduling-and-Frequency.html
        thread::sleep(Duration::from_millis(2) - start_iter.elapsed());
    }
}
