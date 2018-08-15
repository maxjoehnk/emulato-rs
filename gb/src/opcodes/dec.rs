use gameboy::GameBoy;
use opcodes::opcode::OpCode;
use register::Flags;
use std::fmt;
use register::Register8;

pub struct DecrementRegister(pub Register8);

impl fmt::Debug for DecrementRegister {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DEC {:?}", self.0)
    }
}

impl OpCode for DecrementRegister {
    fn exec(&self, gb: &mut GameBoy) {
        {
            let (before, after) = {
                let register = gb.register.get_mut(&self.0);
                let before = *register;
                *register -= 1;
                let after = *register;
                (before, after)
            };
            let half_carry = false; // TODO: get half carry bit
            let result = after == 0u8;
            gb.register.f.set(Flags::Z, result);
            gb.register.f.set(Flags::N, true);
            gb.register.f.set(Flags::H, half_carry);
        }
        pc!(gb);
    }
}