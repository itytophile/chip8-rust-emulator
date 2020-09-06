pub trait OpCode {
    fn execute_opcode(&mut self, opcode: u16) {
        let last_hex = (opcode & 0xF000) >> (3 * 4);

        let x = ((opcode & 0x0F00) >> (2 * 4)) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let n = (opcode & 0x000F) as u8;
        let nn = (opcode & 0x00FF) as u8;
        let nnn = (opcode & 0x0FFF) as usize;

        if opcode == 0x00E0 {
            self.op2()
        }
        if opcode == 0x0EE {
            self.op3()
        }
        match last_hex {
            0x0 => self.op1(),
            0x1 => self.op4(nnn),
            0x2 => self.op5(nnn),
            0x3 => self.op6(x, nn),
            0x4 => self.op7(x, nn),
            0x5 => self.op8(x, y),
            0x6 => self.op9(x, nn),
            0x7 => self.op10(x, nn),
            0x8 => match opcode & 0x000F {
                0x0 => self.op11(x, y),
                0x1 => self.op12(x, y),
                0x2 => self.op13(x, y),
                0x3 => self.op14(x, y),
                0x4 => self.op15(x, y),
                0x5 => self.op16(x, y),
                0x6 => self.op17(x),
                0x7 => self.op18(x, y),
                0xE => self.op19(x),
                _ => panic!("Unknown opcode provided! {:X?}", opcode),
            },
            0x9 => self.op20(x, y),
            0xA => self.op21(nnn),
            0xB => self.op22(nnn),
            0xC => self.op23(x, nn),
            0xD => self.op24(x, y, n),
            0xE => match opcode & 0x00FF {
                0x9E => self.op25(x),
                0xA1 => self.op26(x),
                _ => panic!("Unknown opcode provided! {:X?}", opcode),
            },
            0xF => match opcode & 0x00FF {
                0x07 => self.op27(x),
                0x0A => self.op28(x),
                0x15 => self.op29(x),
                0x18 => self.op30(x),
                0x1E => self.op31(x),
                0x29 => self.op32(x),
                0x33 => self.op33(x),
                0x55 => self.op34(x),
                0x65 => self.op35(x),
                _ => panic!("Unknown opcode provided! {:X?}", opcode),
            },
            _ => panic!("Unknown opcode provided! {:X?}", opcode),
        }
    }
    /// Calls machine code routine (RCA 1802 for COSMAC VIP) at address NNN.
    /// Not necessary for most ROMs.
    ///
    /// # Arguments
    ///
    /// * `opcode` - Opcode 0NNN
    fn op1(&mut self) {
        panic!("Opcode 0NNN not implemented!");
    }
    /// Clears the screen.
    ///
    /// # Arguments
    ///
    /// * `opcode` - Opcode 00E0
    fn op2(&mut self);
    /// Returns from a subroutine.
    ///
    /// # Arguments
    ///
    /// * `opcode` - Opcode 00EE
    fn op3(&mut self);
    /// Jumps to address NNN.
    ///
    /// # Arguments
    ///
    /// * `opcode` - Opcode 1NNN
    fn op4(&mut self, nnn: usize);
    /// Calls subroutine at NNN.
    ///
    /// # Arguments
    ///
    /// * `opcode` - Opcode 2NNN
    fn op5(&mut self, nnn: usize);
    /// Skips the next instruction if VX equals NN
    /// (usually the next instruction is a jump to skip a code block).
    ///
    /// # Arguments
    ///
    /// * `opcode` - Opcode 3XNN
    fn op6(&mut self, x: usize, nn: u8);
    /// Skips the next instruction if VX doesn't equal NN
    /// (usually the next instruction is a jump to skip a code block).
    ///
    /// # Arguments
    ///
    /// * `opcode` - Opcode 4XNN
    fn op7(&mut self, x: usize, nn: u8);
    /// Skips the next instruction if VX equals VY
    /// (usually the next instruction is a jump to skip a code block).
    ///
    /// # Arguments
    ///
    /// * `opcode` - Opcode 5XY0
    fn op8(&mut self, x: usize, y: usize);
    /// Sets VX to NN.
    ///
    /// # Arguments
    ///
    /// * `opcode` - Opcode 6XNN
    fn op9(&mut self, x: usize, nn: u8);
    /// Adds NN to VX (carry flag is not changed).
    ///
    /// # Arguments
    ///
    /// * `opcode` - Opcode 7XNN
    fn op10(&mut self, x: usize, nn: u8);
    /// Sets VX to the value of VY.
    ///
    /// # Arguments
    ///
    /// * `opcode` - Opcode 8XY0
    fn op11(&mut self, x: usize, y: usize);
    /// Sets VX to VX or VY (bitwise OR operation).
    ///
    /// # Arguments
    ///
    /// * `opcode` - Opcode 8XY1
    fn op12(&mut self, x: usize, y: usize);
    /// Sets VX to VX and VY (bitwise AND operation).
    ///
    /// # Arguments
    ///
    /// * `opcode` - Opcode 8XY2
    fn op13(&mut self, x: usize, y: usize);
    /// Sets VX to VX xor VY.
    ///
    /// # Arguments
    ///
    /// * `opcode` - Opcode 8XY3
    fn op14(&mut self, x: usize, y: usize);
    /// Adds VY to VX.
    /// VF is set to 1 when there's a carry, and to 0 when there isn't.
    ///
    /// # Arguments
    ///
    /// * `opcode` - Opcode 8XY4
    fn op15(&mut self, x: usize, y: usize);
    /// VY is subtracted from VX.
    /// VF is set to 0 when there's a borrow, and 1 when there isn't.
    ///
    /// # Arguments
    ///
    /// * `opcode` - Opcode 8XY5
    fn op16(&mut self, x: usize, y: usize);
    /// Stores the least significant bit of VX in VF and then shifts VX to the right by 1.
    ///
    /// # Arguments
    ///
    /// * `opcode` - Opcode 8XY6
    fn op17(&mut self, x: usize);
    /// Sets VX to VY minus VX.
    /// VF is set to 0 when there's a borrow, and 1 when there isn't.
    ///
    /// # Arguments
    ///
    /// * `opcode` - Opcode 8XY7
    fn op18(&mut self, x: usize, y: usize);
    /// Stores the most significant bit of VX in VF and then shifts VX to the left by 1.
    ///
    /// # Arguments
    ///
    /// * `opcode` - Opcode 8XYE
    fn op19(&mut self, x: usize);
    /// Skips the next instruction if VX doesn't equal VY
    /// (usually the next instruction is a jump to skip a code block).
    ///
    /// # Arguments
    ///
    /// * `opcode` - Opcode 9XY0
    fn op20(&mut self, x: usize, y: usize);
    /// Sets I to the address NNN.
    ///
    /// # Arguments
    ///
    /// * `opcode` - Opcode ANNN
    fn op21(&mut self, nnn: usize);
    /// Jumps to the address NNN plus V0.
    ///
    /// # Arguments
    ///
    /// * `opcode` - Opcode BNNN
    fn op22(&mut self, nnn: usize);
    /// Sets VX to the result of a bitwise and
    /// operation on a random number (typically: 0 to 255) and NN.
    ///
    /// # Arguments
    ///
    /// * `opcode` - Opcode CXNN
    fn op23(&mut self, x: usize, nn: u8);
    /// Draws a sprite at coordinate (VX, VY)
    /// that has a width of 8 pixels and a height of N pixels.
    /// Each row of 8 pixels is read as bit-coded starting from memory location I;
    /// I value doesn’t change after the execution of this instruction.
    /// As described above, VF is set to 1 if any screen pixels are flipped from
    /// set to unset when the sprite is drawn, and to 0 if that doesn’t happen.
    ///
    /// # Arguments
    ///
    /// * `opcode` - Opcode DXYN
    fn op24(&mut self, x: usize, y: usize, n: u8);
    /// Skips the next instruction if the key stored in VX is pressed
    /// (usually the next instruction is a jump to skip a code block).
    ///
    /// # Arguments
    ///
    /// * `opcode` - Opcode EX9E
    fn op25(&mut self, x: usize);
    /// Skips the next instruction if the key stored in VX isn't pressed
    /// (usually the next instruction is a jump to skip a code block).
    ///
    /// # Arguments
    ///
    /// * `opcode` - Opcode EXA1
    fn op26(&mut self, x: usize);
    /// Sets VX to the value of the delay timer.
    ///
    /// # Arguments
    ///
    /// * `opcode` - Opcode FX07
    fn op27(&mut self, x: usize);
    /// A key press is awaited, and then stored in VX
    /// (blocking operation, all instruction halted until next key event).
    ///
    /// # Arguments
    ///
    /// * `opcode` - Opcode FX0A
    fn op28(&mut self, x: usize);
    /// Sets the delay timer to VX.
    ///
    /// # Arguments
    ///
    /// * `opcode` - Opcode FX15
    fn op29(&mut self, x: usize);
    /// Sets the sound timer to VX.
    ///
    /// # Arguments
    ///
    /// * `opcode` - Opcode FX18
    fn op30(&mut self, x: usize);
    /// Adds VX to I. VF is not affected.
    ///
    /// # Arguments
    ///
    /// * `opcode` - Opcode FX1E
    fn op31(&mut self, x: usize);
    /// Sets I to the location of the sprite for the character in VX.
    /// Characters 0-F (in hexadecimal) are represented by a 4x5 font.
    ///
    /// # Arguments
    ///
    /// * `opcode` - Opcode FX29
    fn op32(&mut self, x: usize);
    /// Stores the binary-coded decimal representation of VX,
    /// with the most significant of three digits at the address in I,
    /// the middle digit at I plus 1, and the least significant digit at I plus 2.
    /// In other words, take the decimal representation of VX,
    /// place the hundreds digit in memory at location in I,
    /// the tens digit at location I+1, and the ones digit at location I+2.
    ///
    /// # Arguments
    ///
    /// * `opcode` - Opcode FX33
    fn op33(&mut self, x: usize);
    /// Stores V0 to VX (including VX) in memory starting at address I.
    /// The offset from I is increased by 1 for each value written,
    /// but I itself is left unmodified.
    ///
    /// # Arguments
    ///
    /// * `opcode` - Opcode FX55
    fn op34(&mut self, x: usize);
    /// Fills V0 to VX (including VX) with values from memory starting at address I.
    /// The offset from I is increased by 1 for each value written,
    /// but I itself is left unmodified.
    ///
    /// # Arguments
    ///
    /// * `opcode` - Opcode FX65
    fn op35(&mut self, x: usize);
}