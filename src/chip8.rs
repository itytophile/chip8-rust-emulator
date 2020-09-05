mod opcode;

const RAM_SIZE: usize = 4096;
const REGISTER_SIZE: usize = 16;
const STACK_SIZE: usize = 16;
const OFFSET_USABLE_MEM: usize = 0x200;
const SCREEN_WIDTH: u32 = 64;
const SCREEN_HEIGHT: u32 = 32;

pub struct Chip8 {
    ram: [u8; RAM_SIZE],
    v: [u8; REGISTER_SIZE], // registers
    i: usize, // address register
    stack: Vec<u8>,
    delay_timer: u8,
    sound_timer: u8,
    pc: usize, // program counter
}

impl opcode::OpCode for Chip8 {}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            ram: [0; RAM_SIZE],
            v: [0; REGISTER_SIZE],
            i: 0,
            stack: vec![0; STACK_SIZE],
            delay_timer: 0,
            sound_timer: 0,
            pc: OFFSET_USABLE_MEM,
        }
    }

    fn timer_countdown(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    fn get_opcode(&self) -> u16 {
        (self.ram[self.pc] as u16) << 8 + self.ram[self.pc+1]
    }
}