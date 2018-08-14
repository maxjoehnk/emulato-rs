use gameboy::GameBoy;
use opcodes::opcode::OpCode;
use register::Flags;
use std::fmt;
use register::TargetRegister;

pub struct BIT {
    pub bit: u8,
    pub register: TargetRegister
}

impl fmt::Debug for BIT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BIT {:b},{:?}", self.bit, self.register)
    }
}

impl OpCode for BIT {
    fn exec(&self, gb: &mut GameBoy) {
        let data = match self.register {
            TargetRegister::H => gb.register.h,
            _ => panic!()
        };
        let result = data & self.bit == 0;
        gb.register.f.remove(Flags::N);
        gb.register.f.set(Flags::H, true);
        gb.register.f.set(Flags::Z, result);
        pc!(gb, 2);
    }
}