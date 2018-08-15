use gameboy::GameBoy;
use opcodes::opcode::OpCode;
use register::Flags;
use std::fmt;
use register::RegisterPair;

pub struct Push(pub RegisterPair);

impl fmt::Debug for Push {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PUSH {:?}", self.0)
    }
}

impl OpCode for Push {
    fn exec(&self, gb: &mut GameBoy) {
        let register = gb.register.pair(&self.0);
        gb.push_to_stack(register);
        pc!(gb)
    }
}