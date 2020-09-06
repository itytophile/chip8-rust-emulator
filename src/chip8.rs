mod opcode;

use rand::prelude::*;

const RAM_SIZE: usize = 4096;
const REGISTER_SIZE: usize = 16;
const STACK_SIZE: usize = 16;
const OFFSET_USABLE_MEM: usize = 0x200;
const SCREEN_WIDTH: u32 = 64;
const SCREEN_HEIGHT: u32 = 32;

pub struct Chip8 {
    ram: [u8; RAM_SIZE],
    v: [u8; REGISTER_SIZE], // registers
    i: usize,               // address register
    stack: Vec<usize>,
    delay_timer: u8,
    sound_timer: u8,
    pc: usize, // program counter

}

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
        (self.ram[self.pc] as u16) << 8 + self.ram[self.pc + 1]
    }

    fn skip_next_instruction(&mut self) {
        self.pc += 2;
    }

    fn is_key_pressed(key: u8) -> bool {
        // TODO
        println!("Checking if key {} is pressed.", key);
        return false;
    }

    fn get_pressed_key() -> u8 {
        // TODO
        0
    }

    fn get_sprite_address(character: u8) -> usize {
        //TODO
        println!("Getting address of {}.", character);
        0
    }
}

impl opcode::OpCode for Chip8 {
    fn op2(&mut self) {
        panic!("Opcode 00E0 not implemented!");
    }
    fn op3(&mut self) {
        let option = self.stack.pop();
        self.pc = match option {
            Some(n) => n,
            None => panic!("Returning from subroutine failed: the stack is empty!"),
        };
    }
    fn op4(&mut self, nnn: usize) {
        self.pc = nnn;
    }
    fn op5(&mut self, nnn: usize) {
        self.stack.push(self.pc);
        self.op4(nnn);
    }
    fn op6(&mut self, x: usize, nn: u8) {
        if self.v[x] == nn {
            self.skip_next_instruction();
        }
    }
    fn op7(&mut self, x: usize, nn: u8) {
        if self.v[x] != nn {
            self.skip_next_instruction();
        }
    }
    fn op8(&mut self, x: usize, y: usize) {
        if self.v[x] == self.v[y] {
            self.skip_next_instruction();
        }
    }
    fn op9(&mut self, x: usize, nn: u8) {
        self.v[x] = nn;
    }
    fn op10(&mut self, x: usize, nn: u8) {
        self.v[x] += nn;
    }
    fn op11(&mut self, x: usize, y: usize) {
        self.v[x] = self.v[y];
    }
    fn op12(&mut self, x: usize, y: usize) {
        self.v[x] |= self.v[y];
    }
    fn op13(&mut self, x: usize, y: usize) {
        self.v[x] &= self.v[y];
    }
    fn op14(&mut self, x: usize, y: usize) {
        self.v[x] ^= self.v[y];
    }
    fn op15(&mut self, x: usize, y: usize) {
        if (self.v[x] as u16 + self.v[y] as u16) > 0xFF {
            self.v[0xF] = 1;
        } else {
            self.v[0xF] = 0;
        }

        self.v[x] += self.v[y];
    }
    fn op16(&mut self, x: usize, y: usize) {
        if self.v[x] < self.v[y] {
            self.v[0xF] = 0;
        } else {
            self.v[0xF] = 1;
        }

        self.v[x] -= self.v[y];
    }
    fn op17(&mut self, x: usize) {
        self.v[0xF] = self.v[x] & 0b1;
        self.v[x] >>= 1;
    }
    fn op18(&mut self, x: usize, y: usize) {
        if self.v[x] > self.v[y] {
            self.v[0xF] = 0;
        } else {
            self.v[0xF] = 1;
        }

        self.v[x] = self.v[y] - self.v[x];
    }
    fn op19(&mut self, x: usize) {
        self.v[0xF] = self.v[x] & 0b1000_0000;
        self.v[x] <<= 1;
    }
    fn op20(&mut self, x: usize, y: usize) {
        if self.v[x] != self.v[y] {
            self.skip_next_instruction();
        }
    }
    fn op21(&mut self, nnn: usize) {
        self.i = nnn;
    }
    fn op22(&mut self, nnn: usize) {
        self.pc = nnn + self.v[0] as usize;
    }
    fn op23(&mut self, x: usize, nn: u8) {
        self.v[x] = nn & thread_rng().gen_range(0,255);
    }
    fn op24(&mut self, x: usize, y: usize, n: u8) {
        println!("Draw sprite {} {} {}", x, y, n);
    }
    fn op25(&mut self, x: usize) {
        if Chip8::is_key_pressed(self.v[x]) {
            self.skip_next_instruction();
        }
    }
    fn op26(&mut self, x: usize) {
        if !Chip8::is_key_pressed(self.v[x]) {
            self.skip_next_instruction();
        }
    }
    fn op27(&mut self, x: usize) {
        self.v[x] = self.delay_timer;
    }
    fn op28(&mut self, x: usize) {
        self.v[x] = Chip8::get_pressed_key();
    }
    fn op29(&mut self, x: usize) {
        self.delay_timer = self.v[x];
    }
    fn op30(&mut self, x: usize) {
        self.sound_timer = self.v[x];
    }
    fn op31(&mut self, x: usize) {
        self.i += self.v[x] as usize;
    }
    fn op32(&mut self, x: usize) {
        self.i = Chip8::get_sprite_address(self.v[x]);
    }
    fn op33(&mut self, x: usize) {
    }
    fn op34(&mut self, x: usize) {
        for offset in 0..x {
            self.ram[self.i+offset] = self.v[offset];
        }
    }
    fn op35(&mut self, x: usize) {
        for offset in 0..x {
            self.v[offset] = self.ram[self.i+offset];
        }
    }
}
