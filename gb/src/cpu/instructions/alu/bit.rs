use gameboy::GameBoy;
use cpu::Instruction;
use std::fmt;
use cpu::register::{Register8, Flags};

pub struct Bit {
    pub bit: u8,
    pub register: Register8
}

impl fmt::Debug for Bit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BIT {:b},{:?}", self.bit, self.register)
    }
}

impl Instruction for Bit {
    fn exec(&self, gb: &mut GameBoy) {
        let data = gb.register.read_8bit_register(&self.register);
        let result = data & self.bit == 0;
        gb.register.f.remove(Flags::N);
        gb.register.f.set(Flags::H, true);
        gb.register.f.set(Flags::Z, result);
        pc!(gb, 2);
    }
}