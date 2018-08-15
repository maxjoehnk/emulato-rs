use gameboy::GameBoy;
use opcodes::opcode::OpCode;
use register::Flags;
use std::fmt;
use register::TargetRegister;

pub struct XOR(TargetRegister);

impl fmt::Debug for XOR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "XOR {:?}", self.0)
    }
}

impl XOR {
    pub fn new(opcode: u8) -> XOR {
        let register = match opcode {
            0xAF => TargetRegister::A,
            0xA8 => TargetRegister::B,
            0xA9 => TargetRegister::C,
            0xAA => TargetRegister::D,
            0xAB => TargetRegister::E,
            0xAC => TargetRegister::H,
            0xAD => TargetRegister::L,
            // 0xAE => XOR::HL,
            _ => panic!()
        };
        XOR(register)
    }
}

impl OpCode for XOR {
    fn exec(&self, gb: &mut GameBoy) {
        let param = gb.register.read_target(&self.0);
        let result = gb.register.a ^ param;
        gb.register.a = result;
        gb.register.f = if result == 0 {
            Flags::Z
        } else {
            Flags::empty()
        };
        pc!(gb);
    }
}