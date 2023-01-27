use crate::{registers::Reg, SCREEN_HEIGHT, SCREEN_WIDTH};
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
    LDStoreBCD(Reg),
    LDISPRITE(u8),
    ADDI(Reg),
    LDSTREG(Reg),
    LDDTREG(Reg),
    LDREGKEY(Reg),
    LDREGDT(Reg),
    SKNP(Reg),
    SKP(Reg),
    DRW(Reg, Reg, u8),
    RND(Reg, u8),
    LDIAddr(u16),
    SHL(Reg, Reg),
    SUBN(Reg, Reg),
    SHR(Reg, Reg),
    SUB(Reg, Reg),
    XOR(Reg, Reg),
    AND(Reg, Reg),
    OR(Reg, Reg),
    LDREGS(Reg, Reg),
    ADD(Reg, u8),
    LDREGByte(Reg, u8),
    SEREGS(Reg, Reg),
    SNE(Reg, u8),
    SEByte(Reg, u8),
    CALL(u16),
    ADDCARRIED(Reg, Reg),
    SNEREG(Reg, Reg),
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
            0x30 => Ok(Instruction::SEByte(Reg(opcode.0 & 0x0F), opcode.1)),
            0x40 => Ok(Instruction::SNE(Reg(opcode.0 & 0x0F), opcode.1)),
            0x50 => Ok(Instruction::SEREGS(
                Reg(opcode.0 & 0x0F),
                Reg(opcode.1 >> 4),
            )),
            0x60 => Ok(Instruction::LDREGByte(Reg(opcode.0 & 0x0F), opcode.1)),
            0x70 => Ok(Instruction::ADD(Reg(opcode.0 & 0x0F), opcode.1)),
            0x80 => match opcode.1 & 0xF {
                0x0 => Ok(Instruction::LDREGS(
                    Reg(opcode.0 & 0x0F),
                    Reg(opcode.1 >> 4),
                )),
                0x1 => Ok(Instruction::OR(Reg(opcode.0 & 0x0F), Reg(opcode.1 >> 4))),
                0x2 => Ok(Instruction::AND(Reg(opcode.0 & 0x0F), Reg(opcode.1 >> 4))),
                0x3 => Ok(Instruction::XOR(Reg(opcode.0 & 0x0F), Reg(opcode.1 >> 4))),
                0x4 => Ok(Instruction::ADDCARRIED(
                    Reg(opcode.0 & 0x0F),
                    Reg(opcode.1 >> 4),
                )),
                0x5 => Ok(Instruction::SUB(Reg(opcode.0 & 0x0F), Reg(opcode.1 >> 4))),
                0x6 => Ok(Instruction::SHR(Reg(opcode.0 & 0x0F), Reg(opcode.1 >> 4))),
                0x7 => Ok(Instruction::SUBN(Reg(opcode.0 & 0x0F), Reg(opcode.1 >> 4))),
                0xE => Ok(Instruction::SHL(Reg(opcode.0 & 0x0F), Reg(opcode.1 >> 4))),
                _ => Err(format!(
                    "Invalid Instruction Inside 0x8 {:X}{:X}",
                    opcode.0, opcode.1
                )
                .into()),
            },
            0x90 => Ok(Instruction::SNEREG(
                Reg(opcode.0 & 0x0F),
                Reg(opcode.1 >> 4),
            )),
            0xA0 => Ok(Instruction::LDIAddr(Xnnn!(opcode.0, opcode.1))),
            0xB0 => Ok(Instruction::V0JP(Xnnn!(opcode.0, opcode.1))),
            0xC0 => Ok(Instruction::RND(Reg(opcode.0 & 0x0F), opcode.1)),
            0xD0 => Ok(Instruction::DRW(
                Reg(opcode.0 & 0x0F),
                Reg((opcode.1 >> 4) & 0x0F),
                opcode.1 & 0x0F,
            )),
            0xE0 => match opcode.1 {
                0x9E => Ok(Instruction::SKP(Reg(opcode.0 & 0x0F))),
                0xA1 => Ok(Instruction::SKNP(Reg(opcode.0 & 0x0F))),
                _ => {
                    Err(format!("Invalid Instruction Inside E {:X}{:X}", opcode.0, opcode.1).into())
                }
            },
            0xF0 => match opcode.1 {
                0x7 => Ok(Instruction::LDREGDT(Reg(opcode.0 & 0xF))),
                0xA => Ok(Instruction::LDREGKEY(Reg(opcode.0 & 0xF))),
                0x15 => Ok(Instruction::LDDTREG(Reg(opcode.0 & 0xF))),
                0x18 => Ok(Instruction::LDSTREG(Reg(opcode.0 & 0xF))),
                0x1E => Ok(Instruction::ADDI(Reg(opcode.0 & 0xF))),
                0x29 => Ok(Instruction::LDISPRITE(opcode.0 & 0xF)),
                0x33 => Ok(Instruction::LDStoreBCD(Reg(opcode.0 & 0xF))),
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
    dbg!(format!("{instruction:?}, {:X}", emulator.pc - 0x200));
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
            for (i, byte) in data.iter().enumerate() {
                emulator.registers[i as u8] = Reg(*byte);
            }
        }
        Instruction::LDStoreRegisters(v_count) => {
            let init = emulator.i as usize;
            let data = emulator
                .memory
                .get_mut(init..init + v_count as usize)
                .ok_or("I Pointer got out of bound")?;
            for (i, byte) in emulator.registers[..v_count].iter().enumerate() {
                data[i] = byte.0;
            }
        }
        Instruction::LDStoreBCD(vx) => {
            // TODO: Test, not sure if works
            let bcd = emulator.registers[vx];
            emulator.memory[emulator.i as usize] = bcd.0 / 100;
            emulator.memory[emulator.i as usize + 1] = bcd.0 % 100 / 10;
            emulator.memory[emulator.i as usize + 2] = bcd.0 % 10;
        }
        Instruction::LDISPRITE(font_index) => emulator.i = font_index as u16 * 5,
        Instruction::ADDI(vx) => emulator.i += emulator.registers[vx].0 as u16,
        Instruction::LDSTREG(vx) => emulator.sound_timer = emulator.registers[vx].0,
        Instruction::LDDTREG(vx) => emulator.delay_timer = emulator.registers[vx].0,
        Instruction::LDREGKEY(vx) => {
            let keypad = emulator.keypad;
            if keypad == 0 {
                emulator.pc -= 2;
                return Ok(());
            }
            dbg!(keypad);

            for i in 0..0x10 {
                if (1 >> i & keypad) != 0 {
                    emulator.registers[vx] = Reg(i);
                    break;
                }
            }
        }
        Instruction::LDREGDT(vx) => emulator.registers[vx] = Reg(emulator.delay_timer),
        Instruction::SKNP(vx) => {
            if (1 >> emulator.registers[vx].0 & emulator.keypad) == 0 {
                emulator.advance();
            }
        }
        Instruction::SKP(vx) => {
            if (1 >> emulator.registers[vx].0 & emulator.keypad) != 0 {
                emulator.advance();
            }
        }
        Instruction::DRW(vx, vy, n) => {
            drw(emulator, vx, vy, n)?;
        }
        Instruction::RND(vx, max) => emulator.registers[vx] = Reg(rand::random::<u8>() & max),
        Instruction::LDIAddr(addr) => emulator.i = addr,
        Instruction::V0JP(addr) => {
            emulator.pc = addr + emulator.registers[0].0 as u16;
        }
        Instruction::SHL(vx, _) => {
            emulator.registers[0xF] = Reg((emulator.registers[vx].0 & 0b1000000 != 0) as u8);
            emulator.registers[vx].0 <<= 1;
        }
        Instruction::SUBN(vx, vy) => {
            let x = emulator.registers[vx];
            let y = emulator.registers[vy];
            emulator.registers[0xF] = Reg((y > x) as u8);
            emulator.registers[vx] = Reg(y.0 - x.0);
        }
        Instruction::SHR(vx, _) => {
            emulator.registers[0xF] = Reg(vx.0 & 0b1);
            emulator.registers[vx].0 >>= 1;
        }
        Instruction::SUB(vx, vy) => {
            let x = emulator.registers[vx];
            let y = emulator.registers[vy];
            emulator.registers[0xF] = Reg((x > y) as u8);
            emulator.registers[vx].0 -= y.0;
        }
        Instruction::XOR(vx, vy) => {
            emulator.registers[vx].0 ^= emulator.registers[vy].0;
        }
        Instruction::AND(vx, vy) => {
            emulator.registers[vx].0 &= emulator.registers[vy].0;
        }
        Instruction::OR(vx, vy) => {
            emulator.registers[vx].0 |= emulator.registers[vy].0;
        }
        Instruction::LDREGByte(vx, val) => emulator.registers[vx] = Reg(val),
        Instruction::ADD(vx, val) => {
            let (res, overflow) = emulator.registers[vx].0.overflowing_add(val);
            emulator.registers[vx] = Reg(res);
            emulator.registers[0xF] = Reg(overflow as u8);
        }
        Instruction::ADDCARRIED(vx, vy) => {
            let (res, overflow) = emulator.registers[vx]
                .0
                .overflowing_add(emulator.registers[vy].0);
            emulator.registers[vx] = Reg(res);
            emulator.registers[0xF] = Reg(overflow as u8);
        }
        Instruction::LDREGS(vx, vy) => {
            emulator.registers[vx] = emulator.registers[vy];
        }
        Instruction::SEREGS(vx, val) => {
            if emulator.registers[vx] == val {
                emulator.advance()
            }
        }
        Instruction::SNE(vx, val) => {
            if emulator.registers[vx] != Reg(val) {
                emulator.advance()
            }
        }
        Instruction::SEByte(vx, val) => {
            if emulator.registers[vx] == Reg(val) {
                emulator.advance();
            }
        }
        Instruction::CALL(addr) => {
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

// TODO: Think about something better than looping bits, yach
fn drw(emulator: &mut Chip8, vx: Reg, vy: Reg, n: u8) -> Result<(), Box<dyn Error>> {
    let x_pos = emulator.registers[vx];
    let y_pos = emulator.registers[vy];
    let mut collision = false;
    let n = n & 15;
    let sprite = emulator
        .memory
        .get(emulator.i as usize..(emulator.i + n as u16) as usize)
        .ok_or("I pointer got out of bound")?;

    for (i, pixel) in sprite.iter().enumerate() {
        let row = (y_pos.0 as usize + i) % SCREEN_HEIGHT;
        for bit in 0..8 {
            // TODO: somehow make col 0..8 into a united slice
            let col = (x_pos.0 + bit) as usize % SCREEN_WIDTH;
            let location = row * SCREEN_WIDTH + col;
            let new_pixel = (pixel & (0b1 << (7 - bit))) != 0;

            // If the xor going to erase the pixel (1^1), turn on the collision VF
            if new_pixel & emulator.display.data()[location] {
                collision = true;
            }
            emulator.display.mut_data_to_update()[location] ^= new_pixel;
        }
    }
    emulator.registers[0xF] = Reg(collision as u8);
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
        chip.registers[0] = Reg(2);
        chip.registers[1] = Reg(3);
        chip.i = ROM_START as _;
        chip.memory[ROM_START..ROM_START + 4].copy_from_slice(&[255, 0, 255, 255]);
        execute_instruction(&mut chip, Instruction::DRW(Reg(0), Reg(1), 4)).unwrap();
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
        execute_instruction(&mut chip, Instruction::LDISPRITE(3)).unwrap();
        assert_eq!(chip.i, 5 * 3);
        assert_eq!(
            &chip.memory[chip.i as usize..chip.i as usize + 5],
            &[0xF0, 0x10, 0xF0, 0x10, 0xF0]
        );
    }
}
