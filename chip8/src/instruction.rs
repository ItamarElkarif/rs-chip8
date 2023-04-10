use crate::{registers::RegIndex, SCREEN_HEIGHT, SCREEN_WIDTH};
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
    LDStoreBCD(RegIndex),
    LDISPRITE(RegIndex),
    ADDI(RegIndex),
    LDSTREG(RegIndex),
    LDDTREG(RegIndex),
    LDREGKEY(RegIndex),
    LDREGDT(RegIndex),
    SKNP(RegIndex),
    SKP(RegIndex),
    DRW(RegIndex, RegIndex, u8),
    RND(RegIndex, u8),
    LDIAddr(u16),
    SHL(RegIndex, RegIndex),
    SUBN(RegIndex, RegIndex),
    SHR(RegIndex, RegIndex),
    SUB(RegIndex, RegIndex),
    XOR(RegIndex, RegIndex),
    AND(RegIndex, RegIndex),
    OR(RegIndex, RegIndex),
    LDREGS(RegIndex, RegIndex),
    ADD(RegIndex, u8),
    LDREGByte(RegIndex, u8),
    SEREGS(RegIndex, RegIndex),
    SNE(RegIndex, u8),
    SEByte(RegIndex, u8),
    CALL(u16),
    ADDCARRIED(RegIndex, RegIndex),
    SNEREG(RegIndex, RegIndex),
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
            0x30 => Ok(Instruction::SEByte(RegIndex(opcode.0 & 0x0F), opcode.1)),
            0x40 => Ok(Instruction::SNE(RegIndex(opcode.0 & 0x0F), opcode.1)),
            0x50 => Ok(Instruction::SEREGS(
                RegIndex(opcode.0 & 0x0F),
                RegIndex(opcode.1 >> 4),
            )),
            0x60 => Ok(Instruction::LDREGByte(RegIndex(opcode.0 & 0x0F), opcode.1)),
            0x70 => Ok(Instruction::ADD(RegIndex(opcode.0 & 0x0F), opcode.1)),
            0x80 => match opcode.1 & 0xF {
                0x0 => Ok(Instruction::LDREGS(
                    RegIndex(opcode.0 & 0x0F),
                    RegIndex(opcode.1 >> 4),
                )),
                0x1 => Ok(Instruction::OR(
                    RegIndex(opcode.0 & 0x0F),
                    RegIndex(opcode.1 >> 4),
                )),
                0x2 => Ok(Instruction::AND(
                    RegIndex(opcode.0 & 0x0F),
                    RegIndex(opcode.1 >> 4),
                )),
                0x3 => Ok(Instruction::XOR(
                    RegIndex(opcode.0 & 0x0F),
                    RegIndex(opcode.1 >> 4),
                )),
                0x4 => Ok(Instruction::ADDCARRIED(
                    RegIndex(opcode.0 & 0x0F),
                    RegIndex(opcode.1 >> 4),
                )),
                0x5 => Ok(Instruction::SUB(
                    RegIndex(opcode.0 & 0x0F),
                    RegIndex(opcode.1 >> 4),
                )),
                0x6 => Ok(Instruction::SHR(
                    RegIndex(opcode.0 & 0x0F),
                    RegIndex(opcode.1 >> 4),
                )),
                0x7 => Ok(Instruction::SUBN(
                    RegIndex(opcode.0 & 0x0F),
                    RegIndex(opcode.1 >> 4),
                )),
                0xE => Ok(Instruction::SHL(
                    RegIndex(opcode.0 & 0x0F),
                    RegIndex(opcode.1 >> 4),
                )),
                _ => Err(format!(
                    "Invalid Instruction Inside 0x8 {:X}{:X}",
                    opcode.0, opcode.1
                )
                .into()),
            },
            0x90 => Ok(Instruction::SNEREG(
                RegIndex(opcode.0 & 0x0F),
                RegIndex(opcode.1 >> 4),
            )),
            0xA0 => Ok(Instruction::LDIAddr(Xnnn!(opcode.0, opcode.1))),
            0xB0 => Ok(Instruction::V0JP(Xnnn!(opcode.0, opcode.1))),
            0xC0 => Ok(Instruction::RND(RegIndex(opcode.0 & 0x0F), opcode.1)),
            0xD0 => Ok(Instruction::DRW(
                RegIndex(opcode.0 & 0x0F),
                RegIndex((opcode.1 >> 4) & 0x0F),
                opcode.1 & 0x0F,
            )),
            0xE0 => match opcode.1 {
                0x9E => Ok(Instruction::SKP(RegIndex(opcode.0 & 0x0F))),
                0xA1 => Ok(Instruction::SKNP(RegIndex(opcode.0 & 0x0F))),
                _ => {
                    Err(format!("Invalid Instruction Inside E {:X}{:X}", opcode.0, opcode.1).into())
                }
            },
            0xF0 => match opcode.1 {
                0x7 => Ok(Instruction::LDREGDT(RegIndex(opcode.0 & 0xF))),
                0xA => Ok(Instruction::LDREGKEY(RegIndex(opcode.0 & 0xF))),
                0x15 => Ok(Instruction::LDDTREG(RegIndex(opcode.0 & 0xF))),
                0x18 => Ok(Instruction::LDSTREG(RegIndex(opcode.0 & 0xF))),
                0x1E => Ok(Instruction::ADDI(RegIndex(opcode.0 & 0xF))),
                0x29 => Ok(Instruction::LDISPRITE(RegIndex(opcode.0 & 0xF))),
                0x33 => Ok(Instruction::LDStoreBCD(RegIndex(opcode.0 & 0xF))),
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

pub fn execute(emulator: &mut Chip8, instruction: Instruction) -> Result<(), Box<dyn Error>> {
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
                .get(init..init + v_count as usize + 1)
                .ok_or("I Pointer got out of bound")?;
            for (i, byte) in data.iter().enumerate() {
                emulator.registers[RegIndex(i as u8)] = *byte;
            }
        }
        Instruction::LDStoreRegisters(v_count) => {
            let init = emulator.i as usize;
            let data = emulator
                .memory
                .get_mut(init..init + v_count as usize + 1)
                .ok_or("I Pointer got out of bound")?;
            for (i, byte) in emulator.registers[..RegIndex(v_count + 1)]
                .iter()
                .enumerate()
            {
                data[i] = *byte;
            }
        }
        Instruction::LDStoreBCD(vx) => {
            // TODO: Test, not sure if works
            let bcd = emulator.registers[vx];
            emulator.memory[emulator.i as usize] = bcd / 100;
            emulator.memory[emulator.i as usize + 1] = bcd % 100 / 10;
            emulator.memory[emulator.i as usize + 2] = bcd % 10;
        }
        Instruction::LDISPRITE(vx) => emulator.i = emulator.registers[vx] as u16 * 5,
        Instruction::ADDI(vx) => emulator.i += emulator.registers[vx] as u16,
        Instruction::LDSTREG(vx) => emulator.sound_timer = emulator.registers[vx],
        Instruction::LDDTREG(vx) => emulator.delay_timer = emulator.registers[vx],
        Instruction::LDREGKEY(vx) => {
            let keypad = emulator.keypad;
            if keypad == 0 {
                emulator.pc -= 2;
                return Ok(());
            }

            for i in 0..0x10 {
                if (1 >> i & keypad) != 0 {
                    emulator.registers[vx] = i;
                    break;
                }
            }
        }
        Instruction::LDREGDT(vx) => emulator.registers[vx] = emulator.delay_timer,
        Instruction::SKNP(vx) => {
            if (1 << emulator.registers[vx] & emulator.keypad) == 0 {
                emulator.advance();
            }
        }
        Instruction::SKP(vx) => {
            if (1 >> emulator.registers[vx] & emulator.keypad) != 0 {
                emulator.advance();
            }
        }
        Instruction::DRW(vx, vy, n) => {
            drw(emulator, vx, vy, n)?;
        }
        Instruction::RND(vx, max) => emulator.registers[vx] = rand::random::<u8>() & max,
        Instruction::LDIAddr(addr) => emulator.i = addr,
        Instruction::V0JP(addr) => {
            emulator.pc = addr + emulator.registers[RegIndex(0)] as u16;
        }
        Instruction::SHL(vx, _) => {
            emulator.registers[RegIndex(0xF)] = (emulator.registers[vx] & 0b1000000 != 0) as u8;
            emulator.registers[vx] <<= 1;
        }
        Instruction::SUBN(vx, vy) => {
            let x = emulator.registers[vx];
            let y = emulator.registers[vy];
            emulator.registers[RegIndex(0xF)] = (y > x) as u8;
            emulator.registers[vx] = y - x;
        }
        Instruction::SHR(vx, _) => {
            emulator.registers[RegIndex(0xF)] = emulator.registers[vx] & 0b1;
            emulator.registers[vx] >>= 1;
        }
        Instruction::SUB(vx, vy) => {
            let x = emulator.registers[vx];
            let y = emulator.registers[vy];
            emulator.registers[RegIndex(0xF)] = (x > y) as u8;
            (emulator.registers[vx], _) = emulator.registers[vx].overflowing_sub(y);
        }
        Instruction::XOR(vx, vy) => {
            emulator.registers[vx] ^= emulator.registers[vy];
        }
        Instruction::AND(vx, vy) => {
            emulator.registers[vx] &= emulator.registers[vy];
        }
        Instruction::OR(vx, vy) => {
            emulator.registers[vx] |= emulator.registers[vy];
        }
        Instruction::LDREGByte(vx, val) => emulator.registers[vx] = val,
        Instruction::ADD(vx, val) => {
            let (res, overflow) = emulator.registers[vx].overflowing_add(val);
            emulator.registers[vx] = res;
            emulator.registers[RegIndex(0xF)] = overflow as u8;
        }
        Instruction::ADDCARRIED(vx, vy) => {
            let (res, overflow) = emulator.registers[vx].overflowing_add(emulator.registers[vy]);
            emulator.registers[vx] = res;
            emulator.registers[RegIndex(0xF)] = overflow as u8;
        }
        Instruction::LDREGS(vx, vy) => {
            emulator.registers[vx] = emulator.registers[vy];
        }
        Instruction::SEREGS(vx, vy) => {
            if emulator.registers[vx] == emulator.registers[vy] {
                emulator.advance()
            }
        }
        Instruction::SNE(vx, val) => {
            if emulator.registers[vx] != (val) {
                emulator.advance()
            }
        }
        Instruction::SEByte(vx, val) => {
            if emulator.registers[vx] == val {
                emulator.advance();
            }
        }
        Instruction::CALL(addr) => {
            // Since I'm advancing the pc before the instruction, pc will be at least 2
            emulator.stack.push(emulator.pc)?;
            emulator.pc = addr;
        }
        Instruction::SNEREG(vx, vy) => {
            if emulator.registers[vx] != emulator.registers[vy] {
                emulator.advance();
            }
        }
    };
    Ok(())
}

pub fn read(emulator: &mut Chip8) -> Result<Instruction, Box<dyn Error>> {
    let opcode: (u8, u8) = (
        emulator.memory[emulator.pc as usize],
        emulator.memory[emulator.pc as usize + 1],
    );
    emulator.advance();
    Instruction::try_from(opcode)
}

// TODO: Think about something better than looping bits, yach
fn drw(emulator: &mut Chip8, vx: RegIndex, vy: RegIndex, n: u8) -> Result<(), Box<dyn Error>> {
    let x_pos = emulator.registers[vx];
    let y_pos = emulator.registers[vy];
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
    emulator.registers[RegIndex(0xF)] = collision as u8;
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
        chip.registers[RegIndex(0)] = 2;
        chip.registers[RegIndex(1)] = 3;
        chip.i = ROM_START as _;
        chip.memory[ROM_START..ROM_START + 4].copy_from_slice(&[255, 0, 255, 255]);
        execute(&mut chip, Instruction::DRW(RegIndex(0), RegIndex(1), 4)).unwrap();
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
        chip.registers[RegIndex(0)] = 3;
        execute(&mut chip, Instruction::LDISPRITE(RegIndex(0))).unwrap();
        assert_eq!(chip.i, 5 * 3);
        assert_eq!(
            &chip.memory[chip.i as usize..chip.i as usize + 5],
            &[0xF0, 0x10, 0xF0, 0x10, 0xF0]
        );
    }

    #[test]
    fn test_call_ret() {
        let mut chip = Chip8::new(&[0u8; 3584][..]).unwrap();
        execute(&mut chip, Instruction::LDISPRITE(RegIndex(0))).unwrap();
        assert_eq!(chip.i, 5 * 3);
        assert_eq!(
            &chip.memory[chip.i as usize..chip.i as usize + 5],
            &[0xF0, 0x10, 0xF0, 0x10, 0xF0]
        );
    }
}
