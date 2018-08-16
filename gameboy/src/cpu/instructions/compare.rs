use gameboy::GameBoy;
use cpu::Instruction;
use cpu::register::Flags;
use std::fmt;

pub struct CompareImmediate(pub u8);

impl fmt::Debug for CompareImmediate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CP {:#x?}", self.0)
    }
}

impl Instruction for CompareImmediate {
    fn exec(&self, gb: &mut GameBoy) {
        // Wrapping Decrement
        let register = gb.register.a;

        // Update Flags
        let half_carry = false; // TODO: set half_carry
        gb.register.f.set(Flags::Z, register == self.0);
        gb.register.f.set(Flags::N, true);
        gb.register.f.set(Flags::H, half_carry);
        gb.register.f.set(Flags::C, register < self.0);

        // Increment Program Counter
        pc!(gb, 2);
    }
}