use gameboy::GameBoy;
use cpu::Instruction;
use std::fmt;
use cpu::register::{Flags, Register8};

pub struct Xor(Register8);

impl fmt::Debug for Xor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "XOR {:?}", self.0)
    }
}

impl Xor {
    pub fn new(opcode: u8) -> Xor {
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
        Xor(register)
    }
}

impl Instruction for Xor {
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