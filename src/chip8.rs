mod graphic_engine;
mod opcode;
//pub mod sdl_interface;
pub mod piston_interface;
pub mod vulkano_interface;

use graphic_engine::GraphicEngine;
use opcode::OpCode;
use rand::prelude::*;
//use sdl_interface::SdlInterface;
use piston_interface::PistonInterface;
use vulkano_interface::VulkanoInterface;
use std::time::Duration;

const RAM_SIZE: usize = 4096;
const REGISTER_SIZE: usize = 16;
const STACK_SIZE: usize = 16;
const OFFSET_USABLE_MEM: usize = 0x200;
pub const SCREEN_WIDTH: u32 = 64;
pub const SCREEN_HEIGHT: u32 = 32;
const FREQUENCY: u32 = 60;

pub struct Chip8 {
    ram: [u8; RAM_SIZE],
    v: [u8; REGISTER_SIZE], // registers
    i: usize,               // address register
    stack: Vec<usize>,
    delay_timer: u8,
    sound_timer: u8,
    pc: usize, // program counter
    old_pc: usize,
    is_pc_blocked: bool,
    g_engine: Box<dyn GraphicEngine>,
    is_on: bool,
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
            old_pc: 0,
            is_pc_blocked: false,
            // g_engine: Box::new(SdlInterface::new()),
            // g_engine: Box::new(PistonInterface::new()),
            g_engine: Box::new(VulkanoInterface::new()),
            is_on: true,
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
        ((self.ram[self.pc] as u16) << 8) + self.ram[self.pc + 1] as u16
    }

    fn skip_next_instruction(&mut self) {
        self.pc += 2;
    }

    fn is_key_pressed(key: u8) -> bool {
        todo!()
    }

    fn get_pressed_key() -> u8 {
        todo!()
    }

    fn get_sprite_address(character: u8) -> usize {
        todo!()
    }

    fn next_operation(&mut self) {
        if self.is_pc_blocked {
            self.is_pc_blocked = false;
        } else {
            self.pc += 2;
        }
    }

    fn block_pc(&mut self) {
        self.is_pc_blocked = true;
    }

    fn execute_current_operation(&mut self) {
        let opcode = self.get_opcode();
        println!("${:X?}: {:04X?}", self.pc, opcode);
        self.execute_opcode(opcode);
    }

    pub fn run(&mut self) {
        self.g_engine.init();

        let mut frequency = FREQUENCY;

        while self.g_engine.is_running() {
            if self.is_on {
                self.old_pc = self.pc;
                self.execute_current_operation();
                self.next_operation();

                // infinite loop detection
                if self.old_pc == self.pc {
                    println!("Infinite loop detected, stopping execution!");
                    self.is_on = false;
                }

                self.timer_countdown();
            } else {
                frequency = 5;
            }

            self.g_engine.flush();

            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / frequency));
        }
    }

    pub fn read(&mut self, p: &std::path::Path) {
        println!(
            "Reading file '{}'...",
            p.file_name().unwrap().to_str().unwrap()
        );

        let data = std::fs::read(p).unwrap();

        let mut i = OFFSET_USABLE_MEM;

        for byte in data {
            self.ram[i] = byte;
            i += 1;
        }

        println!("Done! {} bytes read.", i - OFFSET_USABLE_MEM);
    }
}

impl OpCode for Chip8 {
    fn op1(&mut self) {
        println!("Opcode 0NNN, shutting down...");
        self.is_on = false;
    }
    fn op2(&mut self) {
        self.g_engine.clear_screen();
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
        self.block_pc();
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
        // I use this strange operation to simulate an overflow
        self.v[x] = ((self.v[x] as u16 + nn as u16) % 0xFF) as u8;
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

        self.v[x] = ((self.v[x] as u16 + self.v[y] as u16) % 0xFF) as u8;
    }
    fn op16(&mut self, x: usize, y: usize) {
        if self.v[x] < self.v[y] {
            self.v[0xF] = 0;
            self.v[x] += 0xFF - self.v[y]; // overflow simulation
        } else {
            self.v[0xF] = 1;
            self.v[x] -= self.v[y];
        }
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
        self.block_pc();
    }
    fn op23(&mut self, x: usize, nn: u8) {
        self.v[x] = nn & thread_rng().gen_range(0, 255);
    }
    fn op24(&mut self, x: usize, y: usize, n: u8) {
        self.g_engine.draw_sprite(
            self.v[x],
            self.v[y],
            &self.ram[self.i..self.i + 1 + n as usize],
        );
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
        self.ram[self.i] = self.v[x] / 100;
        self.ram[self.i + 1] = (self.v[x] / 10) % 10;
        self.ram[self.i + 2] = self.v[x] % 10;
    }
    fn op34(&mut self, x: usize) {
        for offset in 0..x {
            self.ram[self.i + offset] = self.v[offset];
        }
    }
    fn op35(&mut self, x: usize) {
        for offset in 0..x {
            self.v[offset] = self.ram[self.i + offset];
        }
    }
}
