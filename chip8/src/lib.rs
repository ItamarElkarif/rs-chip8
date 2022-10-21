pub const MEM_SIZE: usize = 4 * 1024;
pub const SCREEN_SIZE: usize = 64 * 32;

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
