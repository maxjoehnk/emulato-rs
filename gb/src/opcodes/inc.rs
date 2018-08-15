use gameboy::GameBoy;
use opcodes::opcode::OpCode;
use register::Flags;
use std::fmt;
use register::Register8;

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

impl OpCode for IncrementRegister {
    fn exec(&self, gb: &mut GameBoy) {
        {
            let (before, after) = {
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