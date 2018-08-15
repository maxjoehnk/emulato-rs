use gameboy::GameBoy;
use opcodes::opcode::OpCode;
use register::Flags;
use std::fmt;
use register::Register8;

pub struct XOR(Register8);

impl fmt::Debug for XOR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "XOR {:?}", self.0)
    }
}

impl XOR {
    pub fn new(opcode: u8) -> XOR {
        let register = match opcode {
            0xAF => Register8::A,
            0xA8 => Register8::B,
            0xA9 => Register8::C,
            0xAA => Register8::D,
            0xAB => Register8::E,
            0xAC => Register8::H,
            0xAD => Register8::L,
            // 0xAE => XOR::HL,
            _ => panic!()
        };
        XOR(register)
    }
}

impl OpCode for XOR {
    fn exec(&self, gb: &mut GameBoy) {
        let param = gb.register.read_8bit_register(&self.0);
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