use byteorder::ReadBytesExt;
use std::{
    fs::File,
    io::{BufReader, Cursor, Read},
    thread,
    time::{Duration, Instant},
};

pub const MEM_SIZE: usize = 4 * 1024;
pub const SCREEN_SIZE: usize = 64 * 32;

mod instruction;
use instruction::Instruction;
mod stack;

pub struct Chip8 {
    memory: [u8; MEM_SIZE],
    display: [bool; SCREEN_SIZE], // TODO: Maybe make a struct with api since it is a 2dim array actually
    pc: usize,
    index_pointer: u16,
    stack: stack::Stack,
    delay_timer: u8, // TODO: Maybe atomic? need to decrement it in another thread
    sound_timer: u8,
    keypads: u16, // TODO: use bitflags
    registers: [u8; 0x10],
}

impl Default for Chip8 {
    fn default() -> Self {
        Self {
            memory: [0; MEM_SIZE],
            display: [false; SCREEN_SIZE],
            pc: Default::default(),
            index_pointer: Default::default(),
            stack: Default::default(),
            delay_timer: Default::default(),
            sound_timer: Default::default(),
            keypads: Default::default(),
            registers: Default::default(),
        }
    }
}

// TODO: replace with frame iterators? how to handle input
fn start(file_name: &std::path::Path) {
    let mut emulator: Chip8 = Default::default();

    // Read file to memory
    let mut file_content = BufReader::new(File::open(file_name).unwrap());
    // start timers (delay and sound) - wrap chip in arc
    // parse file?
    let mut rdr = [0u8; 2];
    loop {
        let start_iter = Instant::now();

        file_content.read_exact(&mut rdr).unwrap();
        let opcode: u16 = rdr[0] as u16 | ((rdr[1] as u16) << 8);
        let inst = Instruction::from(opcode);

        emulator.pc += rdr.len();
        execute_instruction(&mut emulator, inst);

        // Delay iteration of the loop, use 500HZ or https://jackson-s.me/2019/07/13/Chip-8-Instruction-Scheduling-and-Frequency.html
        thread::sleep(Duration::from_millis(2) - start_iter.elapsed());
    }
}

fn execute_instruction(emulator: &mut Chip8, instruction: Instruction) {}
