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

pub struct Chip8 {
    pub(crate) memory: [u8; MEM_SIZE],
    pub(crate) display: [[bool; SCREEN_HEIGHT]; SCREEN_WIDTH], // TODO: Maybe make a struct with api since it is a 2dim array actually
    pub(crate) need_redraw: bool,
    pub(crate) pc: u16,
    pub(crate) ip: u16,
    pub(crate) stack: stack::Stack,
    pub(crate) delay_timer: u8, // TODO: Maybe atomic? need to decrement it in another thread
    pub(crate) sound_timer: u8,
    pub(crate) keypads: u16, // TODO: use bitflags
    pub(crate) registers: [u8; 0x10],
}

impl Default for Chip8 {
    fn default() -> Self {
        Self {
            memory: [0; MEM_SIZE],
            display: [[false; SCREEN_HEIGHT]; SCREEN_WIDTH],
            need_redraw: false,
            pc: Default::default(),
            ip: Default::default(),
            stack: Default::default(),
            delay_timer: Default::default(),
            sound_timer: Default::default(),
            keypads: Default::default(),
            registers: Default::default(),
        }
    }
}

impl Chip8 {
    fn execute_instruction(&mut self, instruction: Instruction) -> Result<(), &'static str> {
        self.need_redraw = false;
        // TODO: guard from invalid jump out of mem or getting incapable register, replace with array.get_mut
        match instruction {
            Instruction::CLS => {
                self.display = [[false; SCREEN_HEIGHT]; SCREEN_WIDTH];
                self.need_redraw = true;
            }
            Instruction::RET => {
                self.pc = self.stack.top().unwrap();
                self.stack.pop()?;
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
                // TODO: enforce n should be less than 15 (max sprite size)
                let sprite = &self.memory[(self.ip as usize..(self.ip + n as u16) as usize)];
                let vx = self.registers[regx as usize];
                let vy = self.registers[regy as usize];
                for (y, p) in sprite.iter().enumerate() {
                    let c = vy as usize + y % SCREEN_HEIGHT;
                    for x in 0..8 {
                        let r = (vx + x) as usize % SCREEN_WIDTH;
                        let new_pixel = (p & (0b1 >> (8 - x))) != 0;

                        // If the xor going to erase the pixel (1^1), turn on the VF
                        if new_pixel & self.display[r][c] {
                            self.registers[0xF] = 1;
                        }
                        self.display[r][c] ^= new_pixel;
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
fn start(file_name: &std::path::Path) {
    let mut emulator: Chip8 = Default::default();

    let mut file_content = BufReader::new(File::open(file_name).unwrap());
    // TODO: start timers (delay and sound) - wrap chip in arc or check how much time passed since update
    // parse file?
    let mut rdr = [0u8; 2];
    loop {
        let start_iter = Instant::now();

        file_content.read_exact(&mut rdr).unwrap();
        let opcode: u16 = rdr[0] as u16 | ((rdr[1] as u16) << 8);
        let inst = Instruction::from(opcode);

        emulator.pc += rdr.len() as u16;
        emulator.execute_instruction(inst);

        // Delay iteration of the loop, use 500HZ or https://jackson-s.me/2019/07/13/Chip-8-Instruction-Scheduling-and-Frequency.html
        thread::sleep(Duration::from_millis(2) - start_iter.elapsed());
    }
}
