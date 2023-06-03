use crate::{display::screen_size, registers::RegI, SCREEN_WIDTH};
use std::{error::Error, time::Duration};

use crate::Chip8;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub enum Instruction {
    CLS,
    RET,
    SysJump(u16),
    JP(u16),
    V0JP(u16),
    ReadRegisters(u8),
    StoreRegisters(u8),
    StoreBCD(RegI),
    LoadISprite(RegI),
    ADDI(RegI),
    LoadSoundTimer(RegI),
    LoadDelayTimer(RegI),
    LoadKeys(RegI),
    StoreDelayTimer(RegI),
    SkipNotPressed(RegI),
    SkipPressed(RegI),
    Draw(RegI, RegI, u8),
    Random(RegI, u8),
    LoadI(u16),
    Shl(RegI, RegI),
    SubNotBorrow(RegI, RegI),
    Shr(RegI, RegI),
    Sub(RegI, RegI),
    Xor(RegI, RegI),
    And(RegI, RegI),
    Or(RegI, RegI),
    LoadRegReg(RegI, RegI),
    Add(RegI, u8),
    LoadRegByte(RegI, u8),
    SkipEqualRegReg(RegI, RegI),
    SkipNotEqual(RegI, u8),
    SkipEqual(RegI, u8),
    Call(u16),
    AddCarried(RegI, RegI),
    SkipNotEqualReg(RegI, RegI),
}

macro_rules! Xnnn {
    ($opcode:expr) => {
        ((Xn!($opcode.0)) as u16) << 8 | $opcode.1 as u16
    };
}

macro_rules! Xn {
    ($opcode:expr) => {
        $opcode & 0x0F
    };
}

impl TryFrom<(u8, u8)> for Instruction {
    type Error = Box<dyn Error>;
    fn try_from(opcode: (u8, u8)) -> Result<Self, Box<dyn Error>> {
        match opcode.0 & 0xF0 {
            0x00 => match opcode.1 {
                0xE0 => Ok(Instruction::CLS),
                0xEE => Ok(Instruction::RET),
                _ => Ok(Instruction::SysJump(Xnnn!(opcode))),
            },
            0x10 => Ok(Instruction::JP(Xnnn!(opcode))),
            0x20 => Ok(Instruction::Call(Xnnn!(opcode))),
            0x30 => Ok(Instruction::SkipEqual(RegI(Xn!(opcode.0)), opcode.1)),
            0x40 => Ok(Instruction::SkipNotEqual(RegI(Xn!(opcode.0)), opcode.1)),
            0x50 => Ok(Instruction::SkipEqualRegReg(
                RegI(Xn!(opcode.0)),
                RegI(opcode.1 >> 4),
            )),
            0x60 => Ok(Instruction::LoadRegByte(RegI(Xn!(opcode.0)), opcode.1)),
            0x70 => Ok(Instruction::Add(RegI(Xn!(opcode.0)), opcode.1)),
            0x80 => match opcode.1 & 0xF {
                0x0 => Ok(Instruction::LoadRegReg(
                    RegI(Xn!(opcode.0)),
                    RegI(opcode.1 >> 4),
                )),
                0x1 => Ok(Instruction::Or(RegI(Xn!(opcode.0)), RegI(opcode.1 >> 4))),
                0x2 => Ok(Instruction::And(RegI(Xn!(opcode.0)), RegI(opcode.1 >> 4))),
                0x3 => Ok(Instruction::Xor(RegI(Xn!(opcode.0)), RegI(opcode.1 >> 4))),
                0x4 => Ok(Instruction::AddCarried(
                    RegI(Xn!(opcode.0)),
                    RegI(opcode.1 >> 4),
                )),
                0x5 => Ok(Instruction::Sub(RegI(Xn!(opcode.0)), RegI(opcode.1 >> 4))),
                0x6 => Ok(Instruction::Shr(RegI(Xn!(opcode.0)), RegI(opcode.1 >> 4))),
                0x7 => Ok(Instruction::SubNotBorrow(
                    RegI(Xn!(opcode.0)),
                    RegI(opcode.1 >> 4),
                )),
                0xE => Ok(Instruction::Shl(RegI(Xn!(opcode.0)), RegI(opcode.1 >> 4))),
                _ => Err(format!(
                    "Invalid Instruction Inside 0x8 {:X}{:X}",
                    opcode.0, opcode.1
                )
                .into()),
            },
            0x90 => Ok(Instruction::SkipNotEqualReg(
                RegI(Xn!(opcode.0)),
                RegI(opcode.1 >> 4),
            )),
            0xA0 => Ok(Instruction::LoadI(Xnnn!(opcode))),
            0xB0 => Ok(Instruction::V0JP(Xnnn!(opcode))),
            0xC0 => Ok(Instruction::Random(RegI(Xn!(opcode.0)), opcode.1)),
            0xD0 => Ok(Instruction::Draw(
                RegI(Xn!(opcode.0)),
                RegI((opcode.1 >> 4) & 0x0F),
                Xn!(opcode.1),
            )),
            0xE0 => match opcode.1 {
                0x9E => Ok(Instruction::SkipPressed(RegI(Xn!(opcode.0)))),
                0xA1 => Ok(Instruction::SkipNotPressed(RegI(Xn!(opcode.0)))),
                _ => {
                    Err(format!("Invalid Instruction Inside E {:X}{:X}", opcode.0, opcode.1).into())
                }
            },
            0xF0 => match opcode.1 {
                0x7 => Ok(Instruction::StoreDelayTimer(RegI(opcode.0 & 0xF))),
                0xA => Ok(Instruction::LoadKeys(RegI(opcode.0 & 0xF))),
                0x15 => Ok(Instruction::LoadDelayTimer(RegI(opcode.0 & 0xF))),
                0x18 => Ok(Instruction::LoadSoundTimer(RegI(opcode.0 & 0xF))),
                0x1E => Ok(Instruction::ADDI(RegI(opcode.0 & 0xF))),
                0x29 => Ok(Instruction::LoadISprite(RegI(opcode.0 & 0xF))),
                0x33 => Ok(Instruction::StoreBCD(RegI(opcode.0 & 0xF))),
                0x55 => Ok(Instruction::StoreRegisters(opcode.0 & 0xF)),
                0x65 => Ok(Instruction::ReadRegisters(opcode.0 & 0xF)),
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

pub fn execute(
    emulator: &mut Chip8,
    instruction: Instruction,
) -> Result<std::time::Duration, Box<dyn Error>> {
    match instruction {
        Instruction::CLS => {
            *emulator.display.mut_data_to_update() = [false; screen_size()];
            Ok(Duration::from_micros(109))
        }

        Instruction::RET => {
            emulator.pc = emulator
                .stack
                .pop()
                .ok_or("Can't return to function, The stack is empty")?;
            Ok(Duration::from_micros(105))
        }

        Instruction::SysJump(_) => unreachable!(),
        Instruction::JP(addr) => {
            emulator.pc = addr;
            Ok(Duration::from_micros(105))
        }

        Instruction::ReadRegisters(v_count) => {
            let data = &emulator
                .memory
                .get(emulator.i..emulator.i + v_count as usize + 1)
                .ok_or("I Pointer got out of bound")?;
            for (i, byte) in data.iter().enumerate() {
                emulator.registers[RegI(i as u8)] = *byte;
            }
            Ok(Duration::from_micros(605))
        }

        Instruction::StoreRegisters(v_count) => {
            let init = emulator.i;
            let mem = emulator
                .memory
                .get_mut(init..init + v_count as usize + 1)
                .ok_or("I Pointer got out of bound")?;
            for (i, byte) in emulator.registers[..RegI(v_count + 1)].iter().enumerate() {
                mem[i] = *byte;
            }
            Ok(Duration::from_micros(605))
        }

        Instruction::StoreBCD(vx) => {
            let bcd = emulator.registers[vx];
            emulator.memory[emulator.i] = bcd / 100;
            emulator.memory[emulator.i + 1] = bcd % 100 / 10;
            emulator.memory[emulator.i + 2] = bcd % 10;
            Ok(Duration::from_micros(927))
        }

        Instruction::LoadISprite(vx) => {
            emulator.i = emulator.registers[vx] as usize * 5;
            Ok(Duration::from_micros(91))
        }

        Instruction::ADDI(vx) => {
            emulator.i += emulator.registers[vx] as usize;
            Ok(Duration::from_micros(86))
        }

        Instruction::LoadSoundTimer(vx) => {
            emulator.sound_timer = emulator.registers[vx];
            Ok(Duration::from_micros(45))
        }

        Instruction::LoadDelayTimer(vx) => {
            emulator.delay_timer = emulator.registers[vx];
            Ok(Duration::from_micros(45))
        }

        Instruction::LoadKeys(vx) => {
            let keypad = emulator.keypad;
            if keypad == 0 {
                emulator.pc -= 2;
                return Ok(Duration::from_micros(200));
            }

            for i in 0..0x10 {
                if (1 << i & keypad) != 0 {
                    emulator.registers[vx] = i;
                    break;
                }
            }
            Ok(Duration::ZERO)
        }

        Instruction::StoreDelayTimer(vx) => {
            emulator.registers[vx] = emulator.delay_timer;
            Ok(Duration::from_micros(45))
        }

        Instruction::SkipNotPressed(vx) => {
            if (1 << emulator.registers[vx] & emulator.keypad) == 0 {
                emulator.advance();
            }
            Ok(Duration::from_micros(73))
        }

        Instruction::SkipPressed(vx) => {
            if (1 << emulator.registers[vx] & emulator.keypad) != 0 {
                emulator.advance();
            }
            Ok(Duration::from_micros(73))
        }

        Instruction::Draw(vx, vy, n) => {
            drw(emulator, vx, vy, n)?;

            Ok(Duration::from_micros(22734))
        }

        Instruction::Random(vx, max) => {
            emulator.registers[vx] = rand::random::<u8>() & max;
            Ok(Duration::from_micros(164))
        }

        Instruction::LoadI(addr) => {
            emulator.i = addr as usize;
            Ok(Duration::from_micros(55))
        }

        Instruction::V0JP(addr) => {
            emulator.pc = addr + emulator.registers[RegI(0)] as u16;
            Ok(Duration::from_micros(105))
        }

        Instruction::Shl(vx, _) => {
            emulator.registers[RegI(0xF)] = (emulator.registers[vx] & 0b1000000 != 0) as u8;
            emulator.registers[vx] <<= 1;
            Ok(Duration::from_micros(200))
        }

        Instruction::SubNotBorrow(vx, vy) => {
            let x = emulator.registers[vx];
            let y = emulator.registers[vy];
            emulator.registers[RegI(0xF)] = (y > x) as u8;
            emulator.registers[vx] = y - x;
            Ok(Duration::from_micros(200))
        }

        Instruction::Shr(vx, _) => {
            emulator.registers[RegI(0xF)] = emulator.registers[vx] & 0b1;
            emulator.registers[vx] >>= 1;
            Ok(Duration::from_micros(200))
        }

        Instruction::Sub(vx, vy) => {
            let x = emulator.registers[vx];
            let y = emulator.registers[vy];
            emulator.registers[RegI(0xF)] = (x > y) as u8;
            (emulator.registers[vx], _) = emulator.registers[vx].overflowing_sub(y);
            Ok(Duration::from_micros(200))
        }

        Instruction::Xor(vx, vy) => {
            emulator.registers[vx] ^= emulator.registers[vy];
            Ok(Duration::from_micros(200))
        }

        Instruction::And(vx, vy) => {
            emulator.registers[vx] &= emulator.registers[vy];
            Ok(Duration::from_micros(200))
        }

        Instruction::Or(vx, vy) => {
            emulator.registers[vx] |= emulator.registers[vy];
            Ok(Duration::from_micros(200))
        }

        Instruction::LoadRegByte(vx, val) => {
            emulator.registers[vx] = val;
            Ok(Duration::from_micros(27))
        }

        Instruction::Add(vx, val) => {
            let (res, _overflow) = emulator.registers[vx].overflowing_add(val);
            emulator.registers[vx] = res;
            // emulator.registers[RegIndex(0xF)] = overflow as u8;
            Ok(Duration::from_micros(45))
        }

        Instruction::AddCarried(vx, vy) => {
            let (res, overflow) = emulator.registers[vx].overflowing_add(emulator.registers[vy]);
            emulator.registers[vx] = res;
            emulator.registers[RegI(0xF)] = overflow as u8;
            Ok(Duration::from_micros(200))
        }

        Instruction::LoadRegReg(vx, vy) => {
            emulator.registers[vx] = emulator.registers[vy];
            Ok(Duration::from_micros(200))
        }

        Instruction::SkipEqualRegReg(vx, vy) => {
            if emulator.registers[vx] == emulator.registers[vy] {
                emulator.advance()
            }
            Ok(Duration::from_micros(73))
        }

        Instruction::SkipNotEqual(vx, val) => {
            if emulator.registers[vx] != (val) {
                emulator.advance()
            }
            Ok(Duration::from_micros(55))
        }

        Instruction::SkipEqual(vx, val) => {
            if emulator.registers[vx] == val {
                emulator.advance();
            }
            Ok(Duration::from_micros(55))
        }

        Instruction::SkipNotEqualReg(vx, vy) => {
            if emulator.registers[vx] != emulator.registers[vy] {
                emulator.advance();
            }
            Ok(Duration::from_micros(73))
        }

        Instruction::Call(addr) => {
            emulator.stack.push(emulator.pc)?;
            emulator.pc = addr;
            Ok(Duration::from_micros(105))
        }
    }
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
fn drw(emulator: &mut Chip8, vx: RegI, vy: RegI, n: u8) -> Result<(), Box<dyn Error>> {
    let x_pos = emulator.registers[vx];
    let y_pos = emulator.registers[vy];
    let mut collision = false;
    let rows = n & 15;

    let sprite = emulator
        .memory
        .get(emulator.i..(emulator.i + rows as usize))
        .ok_or("I pointer got out of bound")?;

    let start_location = x_pos as usize + y_pos as usize * SCREEN_WIDTH;
    for (i, pixel) in sprite.iter().enumerate() {
        for bit in 0..8 {
            let location = (start_location + i * SCREEN_WIDTH + bit) % screen_size();
            let new_pixel = (pixel & (0b1 << (7 - bit))) != 0;

            // If the xor going to erase the pixel (1^1), turn on the collision VF
            if new_pixel & emulator.display.data()[location] {
                collision = true;
            }
            emulator.display.mut_data_to_update()[location] ^= new_pixel;
        }
    }
    emulator.registers[RegI(0xF)] = collision as u8;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_drw() {
        let mut chip = Chip8::new(&[0u8; 3584][..]).unwrap();
        chip.registers[RegI(0)] = 2;
        chip.registers[RegI(1)] = 3;
        chip.i = ROM_START as _;
        chip.memory[ROM_START..ROM_START + 4].copy_from_slice(&[255, 0, 255, 255]);
        execute(&mut chip, Instruction::Draw(RegI(0), RegI(1), 4)).unwrap();
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
        chip.registers[RegI(0)] = 3;
        execute(&mut chip, Instruction::LoadISprite(RegI(0))).unwrap();
        assert_eq!(chip.i, 5 * 3);
        assert_eq!(
            &chip.memory[chip.i..chip.i + 5],
            &[0xF0, 0x10, 0xF0, 0x10, 0xF0]
        );
    }
}
