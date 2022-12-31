use std::error::Error;

// TODO: read how to decode with http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#3.0
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
    LDSetNibbles(u8, u8),
    ADD(u8, u8),
    LD(u8, u8),
    SENibble(u8, u8),
    SNE(u8, u8),
    SEByte(u8, u8),
    CALL(u16),
}

impl TryFrom<u16> for Instruction {
    type Error = Box<dyn Error>;
    fn try_from(opcode: u16) -> Result<Self, Box<dyn Error>> {
        match opcode & 0xF000 {
            0x0000 => match opcode {
                0x00E0 => Ok(Instruction::CLS),
                0x00EE => Ok(Instruction::RET),
                _ => Ok(Instruction::SysJump(opcode & 0xFFF)),
            },
            0x1000 => Ok(Instruction::JP(opcode & 0xFFF)),
            0x2000 => Ok(Instruction::CALL(opcode & 0xFFF)),
            0x3000 => Ok(Instruction::SEByte(
                (opcode >> 8) as u8 & 0x0F,
                opcode as u8,
            )),
            0x4000 => Ok(Instruction::SNE((opcode >> 8) as u8 & 0x0F, opcode as u8)),
            0x5000 => Ok(Instruction::SENibble(
                (opcode >> 8) as u8 & 0x0F,
                opcode as u8 & 0xF0,
            )),
            0x6000 => Ok(Instruction::LD((opcode >> 8) as u8 & 0x0F, opcode as u8)),
            0x7000 => Ok(Instruction::ADD((opcode >> 8) as u8 & 0x0F, opcode as u8)),
            0x8000 => match opcode & 0xF {
                0x0 => Ok(Instruction::LDSetNibbles(
                    (opcode >> 8) as u8 & 0x0F,
                    opcode as u8 & 0xF0,
                )),
                0x1 => Ok(Instruction::OR(
                    (opcode >> 8) as u8 & 0x0F,
                    opcode as u8 & 0xF0,
                )),
                0x2 => Ok(Instruction::AND(
                    (opcode >> 8) as u8 & 0x0F,
                    opcode as u8 & 0xF0,
                )),
                0x3 => Ok(Instruction::XOR(
                    (opcode >> 8) as u8 & 0x0F,
                    opcode as u8 & 0xF0,
                )),
                0x4 => Ok(Instruction::ADD(
                    (opcode >> 8) as u8 & 0x0F,
                    opcode as u8 & 0xF0,
                )),
                0x5 => Ok(Instruction::SUB(
                    (opcode >> 8) as u8 & 0x0F,
                    opcode as u8 & 0xF0,
                )),
                0x6 => Ok(Instruction::SHR(
                    (opcode >> 8) as u8 & 0x0F,
                    opcode as u8 & 0xF0,
                )),
                0x7 => Ok(Instruction::SUBN(
                    (opcode >> 8) as u8 & 0x0F,
                    opcode as u8 & 0xF0,
                )),
                0xE => Ok(Instruction::SHL(
                    (opcode >> 8) as u8 & 0x0F,
                    opcode as u8 & 0xF0,
                )),
                _ => Err(format!("Invalid Instruction Inside 0x8 {:X}", opcode).into()),
            },
            0x9000 => Ok(Instruction::SNE(
                (opcode >> 8) as u8 & 0x0F,
                opcode as u8 & 0xF0,
            )),
            0xA000 => Ok(Instruction::LDSetIAddr(opcode & 0xFFF)),
            0xB000 => Ok(Instruction::V0JP(opcode & 0xFFF)),
            0xC000 => Ok(Instruction::RND((opcode >> 8) as u8 & 0x0F, opcode as u8)),
            0xD000 => Ok(Instruction::DRW(
                (opcode >> 8) as u8 & 0x0F,
                opcode as u8 & 0xF0,
                opcode as u8 & 0x0F,
            )),
            0xE000 => match opcode & 0x00FF {
                0x9E => Ok(Instruction::SKP((opcode >> 8) as u8 & 0x0F)),
                0xA1E => Ok(Instruction::SKNP((opcode >> 8) as u8 & 0x0F)),
                _ => Err(format!("Invalid Instruction Inside E {:X}", opcode).into()),
            },
            0xF000 => match opcode & 0x00FF {
                0x7 => Ok(Instruction::LDGetDT((opcode >> 8) as u8 & 0xF)),
                0xA => Ok(Instruction::LDKeyPress((opcode >> 8) as u8 & 0xF)),
                0x15 => Ok(Instruction::LDSetDT((opcode >> 8) as u8 & 0xF)),
                0x18 => Ok(Instruction::LDSetST((opcode >> 8) as u8 & 0xF)),
                0x1E => Ok(Instruction::ADDI((opcode >> 8) as u8 & 0xF)),
                0x29 => Ok(Instruction::LDSetISprite((opcode >> 8) as u8 & 0xF)),
                0x33 => Ok(Instruction::LDStoreBCD((opcode >> 8) as u8 & 0xF)),
                0x55 => Ok(Instruction::LDStoreRegisters((opcode >> 8) as u8 & 0xF)),
                0x65 => Ok(Instruction::LDReadRegisters((opcode >> 8) as u8 & 0xF)),
                _ => Err(format!("Invalid Instruction Inside F {:X}", opcode).into()),
            },
            _ => Err(format!("Invalid Instruction {:X}", opcode).into()),
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
