use gameboy::GameBoy;
use cpu::Instruction;
use std::fmt;
use cpu::register::{Flags, Register8, Register16};

pub struct IncrementRegister(Register8);

impl IncrementRegister {
    pub fn new(opcode: u8) -> IncrementRegister {
        let register = match opcode {
            0x04 => Register8::B,
            0x0C => Register8::C,
            0x14 => Register8::D,
            0x1C => Register8::E,
            0x24 => Register8::H,
            0x2C => Register8::L,
            0x3C => Register8::A,
            _ => unreachable!()
        };
        IncrementRegister(register)
    }
}

impl fmt::Debug for IncrementRegister {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "INC {:?}", self.0)
    }
}

impl Instruction for IncrementRegister {
    fn exec(&self, gb: &mut GameBoy) {
        {
            let (_before, after) = {
                let register = gb.register.get_mut(&self.0);
                let before = *register;
                *register += 1;
                let after = *register;
                (before, after)
            };
            let half_carry = false; // TODO: get half carry bit
            let result = after == 0u8;
            gb.register.f.set(Flags::Z, result);
            gb.register.f.remove(Flags::N);
            gb.register.f.set(Flags::H, half_carry);
        }
        pc!(gb);
    }
}

pub struct Increment16BitRegister(pub Register16);

impl fmt::Debug for Increment16BitRegister {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "INC {:?}", self.0)
    }
}

impl Instruction for Increment16BitRegister {
    fn exec(&self, gb: &mut GameBoy) {
        let mut value = gb.register.read_16bit_register(&self.0);
        value = value.wrapping_add(1);
        gb.register.write_16bit_register(&self.0, value);
        pc!(gb);
    }
}