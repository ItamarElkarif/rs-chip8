use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};
use std::error::Error;

use crate::Chip8;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub enum Instruction {
    CLS,
    RET,
    SysJump(u16),
    JP(u16),
    V0JP(u16),
    LDReadRegisters(u8),
    LDStoreRegisters(u8),
    LDStoreBCD(u8),
    LDSetISprite(u8),
    ADDI(u8),
    LDSetST(u8),
    LDSetDT(u8),
    LDKeyPress(u8),
    LDGetDT(u8),
    SKNP(u8),
    SKP(u8),
    DRW(u8, u8, u8),
    RND(u8, u8),
    LDSetIAddr(u16),
    SHL(u8, u8),
    SUBN(u8, u8),
    SHR(u8, u8),
    SUB(u8, u8),
    XOR(u8, u8),
    AND(u8, u8),
    OR(u8, u8),
    LDREGS(u8, u8),
    ADD(u8, u8),
    LD(u8, u8),
    SEREG(u8, u8),
    SNE(u8, u8),
    SEByte(u8, u8),
    CALL(u16),
    ADDCARRIED(u8, u8),
    SNEREG(u8, u8),
}

macro_rules! Xnnn {
    ($hi:expr, $lo:expr) => {
        (($hi & 0xF) as u16) << 8 | $lo as u16
    };
}

impl TryFrom<(u8, u8)> for Instruction {
    type Error = Box<dyn Error>;
    fn try_from(opcode: (u8, u8)) -> Result<Self, Box<dyn Error>> {
        match opcode.0 & 0xF0 {
            0x00 => match opcode.1 {
                0xE0 => Ok(Instruction::CLS),
                0xEE => Ok(Instruction::RET),
                _ => Ok(Instruction::SysJump(Xnnn!(opcode.0, opcode.1))),
            },
            0x10 => Ok(Instruction::JP(Xnnn!(opcode.0, opcode.1))),
            0x20 => Ok(Instruction::CALL(Xnnn!(opcode.0, opcode.1))),
            0x30 => Ok(Instruction::SEByte(opcode.0 & 0x0F, opcode.1)),
            0x40 => Ok(Instruction::SNE(opcode.0 & 0x0F, opcode.1)),
            0x50 => Ok(Instruction::SEREG(opcode.0 & 0x0F, opcode.1 >> 4)),
            0x60 => Ok(Instruction::LD(opcode.0 & 0x0F, opcode.1)),
            0x70 => Ok(Instruction::ADD(opcode.0 & 0x0F, opcode.1)),
            0x80 => match opcode.1 & 0xF {
                0x0 => Ok(Instruction::LDREGS(opcode.0 & 0x0F, opcode.1 >> 4)),
                0x1 => Ok(Instruction::OR(opcode.0 & 0x0F, opcode.1 >> 4)),
                0x2 => Ok(Instruction::AND(opcode.0 & 0x0F, opcode.1 >> 4)),
                0x3 => Ok(Instruction::XOR(opcode.0 & 0x0F, opcode.1 >> 4)),
                0x4 => Ok(Instruction::ADDCARRIED(opcode.0 & 0x0F, opcode.1 >> 4)),
                0x5 => Ok(Instruction::SUB(opcode.0 & 0x0F, opcode.1 >> 4)),
                0x6 => Ok(Instruction::SHR(opcode.0 & 0x0F, opcode.1 >> 4)),
                0x7 => Ok(Instruction::SUBN(opcode.0 & 0x0F, opcode.1 >> 4)),
                0xE => Ok(Instruction::SHL(opcode.0 & 0x0F, opcode.1 >> 4)),
                _ => Err(format!(
                    "Invalid Instruction Inside 0x8 {:X}{:X}",
                    opcode.0, opcode.1
                )
                .into()),
            },
            0x90 => Ok(Instruction::SNEREG(opcode.0 & 0x0F, opcode.1 >> 4)),
            0xA0 => Ok(Instruction::LDSetIAddr(Xnnn!(opcode.0, opcode.1))),
            0xB0 => Ok(Instruction::V0JP(Xnnn!(opcode.0, opcode.1))),
            0xC0 => Ok(Instruction::RND(opcode.0 & 0x0F, opcode.1)),
            0xD0 => Ok(Instruction::DRW(
                opcode.0 & 0x0F,
                (opcode.1 >> 4) & 0x0F,
                opcode.1 & 0x0F,
            )),
            0xE0 => match opcode.1 {
                0x9E => Ok(Instruction::SKP(opcode.0 & 0x0F)),
                0xA1 => Ok(Instruction::SKNP(opcode.0 & 0x0F)),
                _ => {
                    Err(format!("Invalid Instruction Inside E {:X}{:X}", opcode.0, opcode.1).into())
                }
            },
            0xF0 => match opcode.1 {
                0x7 => Ok(Instruction::LDGetDT(opcode.0 & 0xF)),
                0xA => Ok(Instruction::LDKeyPress(opcode.0 & 0xF)),
                0x15 => Ok(Instruction::LDSetDT(opcode.0 & 0xF)),
                0x18 => Ok(Instruction::LDSetST(opcode.0 & 0xF)),
                0x1E => Ok(Instruction::ADDI(opcode.0 & 0xF)),
                0x29 => Ok(Instruction::LDSetISprite(opcode.0 & 0xF)),
                0x33 => Ok(Instruction::LDStoreBCD(opcode.0 & 0xF)),
                0x55 => Ok(Instruction::LDStoreRegisters(opcode.0 & 0xF)),
                0x65 => Ok(Instruction::LDReadRegisters(opcode.0 & 0xF)),
                _ => {
                    Err(format!("Invalid Instruction Inside F {:X}{:X}", opcode.0, opcode.1).into())
                }
            },
            _ => Err(format!("Invalid Instruction {:X}{:X}", opcode.0, opcode.1).into()),
            // TODO: add Super chip-48 Instructions
            // 0x0 => Instruction::SCD,
            // 0x0 => Instruction::SCR,
            // 0x0 => Instruction::SCL,
            // 0x0 => Instruction::EXIT,
            // 0x0 => Instruction::LOW,
            // 0x0 => Instruction::HIGH,
            // 0xD => Instruction::DRW,
            // 0xF => Instruction::LD,
            // 0xF => Instruction::LD,
            // 0xF => Instruction::LD,
        }
    }
}

pub fn execute_instruction(
    emulator: &mut Chip8,
    instruction: Instruction,
) -> Result<(), Box<dyn Error>> {
    // TODO: replace registers with Reg(u8) being indexed, problem with range of Regs
    match instruction {
        Instruction::CLS => {
            *emulator.display.mut_data_to_update() = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
        }
        Instruction::RET => {
            emulator.pc = emulator
                .stack
                .pop()
                .ok_or("Can't return root function, the stack is empty")?;
        }
        Instruction::SysJump(_) => unreachable!(),
        Instruction::JP(addr) => {
            emulator.pc = addr;
        }
        Instruction::LDReadRegisters(v_count) => {
            let init = emulator.i as usize;
            let data = &emulator
                .memory
                .get(init..init + v_count as usize)
                .ok_or("I Pointer got out of bound")?;
            emulator.registers[..v_count as usize].copy_from_slice(data);
        }
        Instruction::LDStoreRegisters(v_count) => {
            let init = emulator.i as usize;
            let data = emulator
                .memory
                .get_mut(init..init + v_count as usize)
                .ok_or("I Pointer got out of bound")?;
            data.copy_from_slice(&emulator.registers[..v_count as usize]);
        }
        Instruction::LDStoreBCD(vx) => {
            // TODO: Test, not sure if works
            let bcd = emulator.registers[vx as usize];
            emulator.memory[emulator.i as usize] = bcd / 100;
            emulator.memory[emulator.i as usize + 1] = bcd % 100 / 10;
            emulator.memory[emulator.i as usize + 2] = bcd % 10;
        }
        Instruction::LDSetISprite(vx) => emulator.i = vx as u16 * 5,
        Instruction::ADDI(vx) => emulator.i += emulator.registers[vx as usize] as u16,
        Instruction::LDSetST(vx) => emulator.sound_timer = emulator.registers[vx as usize],
        Instruction::LDSetDT(vx) => emulator.delay_timer = emulator.registers[vx as usize],
        Instruction::LDKeyPress(vx) => emulator.registers[vx as usize] = todo!(), // GetKey
        Instruction::LDGetDT(vx) => emulator.registers[vx as usize] = emulator.delay_timer,
        Instruction::SKNP(_vx) => todo!(),
        Instruction::SKP(_vx) => todo!(),
        Instruction::DRW(vx, vy, n) => {
            drw(emulator, vx, vy, n)?;
        }
        Instruction::RND(vx, max) => emulator.registers[vx as usize] = rand::random::<u8>() & max,
        Instruction::LDSetIAddr(addr) => emulator.i = addr,
        Instruction::V0JP(addr) => {
            emulator.pc = addr + emulator.registers[0] as u16;
        }
        Instruction::SHL(vx, _) => {
            emulator.registers[0xF] = (emulator.registers[vx as usize] & 0b1000000 != 0) as u8;
            emulator.registers[vx as usize] <<= 1;
        }
        Instruction::SUBN(vx, vy) => {
            let x = emulator.registers[vx as usize];
            let y = emulator.registers[vy as usize];
            emulator.registers[0xF] = (y > x) as u8;
            emulator.registers[vx as usize] = y - x;
        }
        Instruction::SHR(vx, _) => {
            emulator.registers[0xF] = vx & 0b1;
            emulator.registers[vx as usize] >>= 1;
        }
        Instruction::SUB(vx, vy) => {
            let x = emulator.registers[vx as usize];
            let y = emulator.registers[vy as usize];
            emulator.registers[0xF] = (x > y) as u8;
            emulator.registers[vx as usize] -= y;
        }
        Instruction::XOR(vx, vy) => {
            emulator.registers[vx as usize] ^= emulator.registers[vy as usize];
        }
        Instruction::AND(vx, vy) => {
            emulator.registers[vx as usize] &= emulator.registers[vy as usize];
        }
        Instruction::OR(vx, vy) => {
            emulator.registers[vx as usize] |= emulator.registers[vy as usize];
        }
        Instruction::LD(vx, val) => emulator.registers[vx as usize] = val,
        Instruction::ADD(vx, val) => {
            emulator.registers[vx as usize] += val;
        }
        Instruction::ADDCARRIED(vx, vy) => {
            let (res, overflow) =
                emulator.registers[vx as usize].overflowing_add(emulator.registers[vy as usize]);
            emulator.registers[vx as usize] = res;
            emulator.registers[0xF] = overflow as u8;
        }
        Instruction::LDREGS(vx, vy) => {
            emulator.registers[vx as usize] = emulator.registers[vy as usize];
        }
        Instruction::SEREG(vx, val) => {
            if emulator.registers[vx as usize] == val {
                emulator.pc += 2
            }
        }
        Instruction::SNE(vx, val) => {
            if emulator.registers[vx as usize] != val {
                emulator.pc += 2
            }
        }
        Instruction::SEByte(vx, val) => {
            if emulator.registers[vx as usize] == val {
                emulator.pc += 1;
            }
        }
        Instruction::CALL(addr) => {
            emulator.stack.push(emulator.pc)?;
            emulator.pc = addr;
        }
        Instruction::SNEREG(vx, vy) => {
            if emulator.registers[vx as usize] != emulator.registers[vy as usize] {
                emulator.pc += 1;
            }
        }
    };
    Ok(())
}

// TODO: Think about something better than looping bits, yach
fn drw(emulator: &mut Chip8, vx: u8, vy: u8, n: u8) -> Result<(), Box<dyn Error>> {
    let x_pos = emulator.registers[vx as usize];
    let y_pos = emulator.registers[vy as usize];
    let mut collision = false;
    let n = n & 15;
    let sprite = emulator
        .memory
        .get(emulator.i as usize..(emulator.i + n as u16) as usize)
        .ok_or("I pointer got out of bound")?;

    for (i, pixel) in sprite.iter().enumerate() {
        let row = (y_pos as usize + i) % SCREEN_HEIGHT;
        for bit in 0..8 {
            // TODO: somehow make col 0..8 into a united slice
            let col = (x_pos + bit) as usize % SCREEN_WIDTH;
            let location = row * SCREEN_WIDTH + col;
            let new_pixel = (pixel & (0b1 << (7 - bit))) != 0;

            // If the xor going to erase the pixel (1^1), turn on the collision VF
            if new_pixel & emulator.display.data()[location] {
                collision = true;
            }
            emulator.display.mut_data_to_update()[location] ^= new_pixel;
        }
    }
    emulator.registers[0xF] = collision as u8;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    struct MockUI;
    impl UI for MockUI {
        fn update(&mut self, _: &DisplayData) {}
        fn beep(&self) {}
    }

    #[test]
    fn test_drw() {
        let mut chip = Chip8::new(&[0u8; 3584][..]).unwrap();
        chip.registers[0] = 2;
        chip.registers[1] = 3;
        chip.i = ROM_START as _;
        chip.memory[ROM_START..ROM_START + 4].copy_from_slice(&[255, 0, 255, 255]);
        execute_instruction(&mut chip, Instruction::DRW(0, 1, 4)).unwrap();
        for row in &[3, 5, 6] {
            assert_eq!(
                chip.display.data()[row * SCREEN_WIDTH + 2..row * SCREEN_WIDTH + 8 + 2],
                [true, true, true, true, true, true, true, true]
            );
        }
    }

    #[test]
    fn test_set_i_sprite() {
        let mut chip = Chip8::new(&[0u8; 3584][..]).unwrap();
        execute_instruction(&mut chip, Instruction::LDSetISprite(3)).unwrap();
        assert_eq!(chip.i, 5 * 3);
        assert_eq!(
            &chip.memory[chip.i as usize..chip.i as usize + 5],
            &[0xF0, 0x10, 0xF0, 0x10, 0xF0]
        );
    }
}
