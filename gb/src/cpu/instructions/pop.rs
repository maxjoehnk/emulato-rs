use gameboy::GameBoy;
use cpu::Instruction;
use std::fmt;
use cpu::register::RegisterPair;

pub struct Pop(pub RegisterPair);

impl fmt::Debug for Pop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "POP {:?}", self.0)
    }
}

impl Instruction for Pop {
    fn exec(&self, gb: &mut GameBoy) {
        let value = gb.pop_from_stack();
        gb.register.write_16bit_register(&self.0.into(), value);
        pc!(gb)
    }
}