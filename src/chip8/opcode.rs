pub trait OpCode {
    fn execute_opcode(&mut self, opcode: u16) {
        let last_hex = (opcode & 0xF000) >> 12;

        if opcode == 0x00E0 {
            self.op2(opcode)
        }
        if opcode == 0x0EE {
            self.op3(opcode)
        }
        match last_hex {
            0x0 => self.op1(opcode),
            0x1 => self.op4(opcode),
            0x2 => self.op5(opcode),
            0x3 => self.op6(opcode),
            0x4 => self.op7(opcode),
            0x5 => self.op8(opcode),
            0x6 => self.op9(opcode),
            0x7 => self.op10(opcode),
            0x8 => match opcode & 0x000F {
                0x0 => self.op11(opcode),
                0x1 => self.op12(opcode),
                0x2 => self.op13(opcode),
                0x3 => self.op14(opcode),
                0x4 => self.op15(opcode),
                0x5 => self.op16(opcode),
                0x6 => self.op17(opcode),
                0x7 => self.op18(opcode),
                0xE => self.op19(opcode),
                _ => panic!("Unknown opcode provided! {}", opcode),
            },
            0x9 => self.op20(opcode),
            0xA => self.op21(opcode),
            0xB => self.op22(opcode),
            0xC => self.op23(opcode),
            0xD => self.op24(opcode),
            0xE => match opcode & 0x00FF {
                0x9E => self.op25(opcode),
                0xA1 => self.op26(opcode),
                _ => panic!("Unknown opcode provided! {}", opcode),
            },
            0xF => match opcode & 0x00FF {
                0x07 => self.op27(opcode),
                0x0A => self.op28(opcode),
                0x15 => self.op29(opcode),
                0x18 => self.op30(opcode),
                0x1E => self.op31(opcode),
                0x29 => self.op32(opcode),
                0x33 => self.op33(opcode),
                0x55 => self.op34(opcode),
                0x65 => self.op35(opcode),
                _ => panic!("Unknown opcode provided! {}", opcode),
            },
            _ => panic!("Unknown opcode provided! {}", opcode),
        }
    }
    fn op1(&mut self, opcode: u16) {}
    fn op2(&mut self, opcode: u16) {}
    fn op3(&mut self, opcode: u16) {}
    fn op4(&mut self, opcode: u16) {}
    fn op5(&mut self, opcode: u16) {}
    fn op6(&mut self, opcode: u16) {}
    fn op7(&mut self, opcode: u16) {}
    fn op8(&mut self, opcode: u16) {}
    fn op9(&mut self, opcode: u16) {}
    fn op10(&mut self, opcode: u16) {}
    fn op11(&mut self, opcode: u16) {}
    fn op12(&mut self, opcode: u16) {}
    fn op13(&mut self, opcode: u16) {}
    fn op14(&mut self, opcode: u16) {}
    fn op15(&mut self, opcode: u16) {}
    fn op16(&mut self, opcode: u16) {}
    fn op17(&mut self, opcode: u16) {}
    fn op18(&mut self, opcode: u16) {}
    fn op19(&mut self, opcode: u16) {}
    fn op20(&mut self, opcode: u16) {}
    fn op21(&mut self, opcode: u16) {}
    fn op22(&mut self, opcode: u16) {}
    fn op23(&mut self, opcode: u16) {}
    fn op24(&mut self, opcode: u16) {}
    fn op25(&mut self, opcode: u16) {}
    fn op26(&mut self, opcode: u16) {}
    fn op27(&mut self, opcode: u16) {}
    fn op28(&mut self, opcode: u16) {}
    fn op29(&mut self, opcode: u16) {}
    fn op30(&mut self, opcode: u16) {}
    fn op31(&mut self, opcode: u16) {}
    fn op32(&mut self, opcode: u16) {}
    fn op33(&mut self, opcode: u16) {}
    fn op34(&mut self, opcode: u16) {}
    fn op35(&mut self, opcode: u16) {}
}
