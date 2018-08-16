use gameboy::GameBoy;
use cpu::Instruction;
use std::fmt;

pub struct Return;

impl fmt::Debug for Return {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RET")
    }
}

impl Instruction for Return {
    fn exec(&self, gb: &mut GameBoy) {
        let next_instruction = gb.pop_from_stack();
        gb.register.pc = next_instruction;
    }
}