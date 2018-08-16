use gameboy::GameBoy;
use cpu::Instruction;
use cpu::register::{Flags, Register8};
use std::fmt;

pub struct DecrementRegister(pub Register8);

impl fmt::Debug for DecrementRegister {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DEC {:?}", self.0)
    }
}

impl Instruction for DecrementRegister {
    fn exec(&self, gb: &mut GameBoy) {
        // Wrapping Decrement
        let register = gb.register.get(&self.0);
        let after = register.wrapping_sub(1);
        gb.register.write_8bit_register(&self.0, after);

        // Update Flags
        let half_carry = false; // TODO: get half carry bit
        let result = after == 0u8;
        gb.register.f.set(Flags::Z, result);
        gb.register.f.set(Flags::N, true);
        gb.register.f.set(Flags::H, half_carry);

        // Increment Program Counter
        pc!(gb);
    }
}