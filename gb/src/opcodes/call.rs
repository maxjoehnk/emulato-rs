use gameboy::GameBoy;
use opcodes::opcode::OpCode;
use register::Flags;
use std::fmt;

pub struct Call(pub u16);

impl fmt::Debug for Call {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CALL {:#X?}", self.0)
    }
}

impl OpCode for Call {
    fn exec(&self, gb: &mut GameBoy) {
        let next_instruction = gb.register.pc + 3;
        gb.push_to_stack(next_instruction);
        gb.register.pc = self.0;
    }
}