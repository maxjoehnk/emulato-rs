use gameboy::GameBoy;
use opcodes::opcode::OpCode;
use register::Flags;
use std::fmt;

pub enum XOR {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    // HL TODO
}

impl fmt::Debug for XOR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            XOR::A => write!(f, "XOR A"),
            XOR::B => write!(f, "XOR B"),
            XOR::C => write!(f, "XOR C"),
            XOR::D => write!(f, "XOR D"),
            XOR::E => write!(f, "XOR E"),
            XOR::H => write!(f, "XOR H"),
            XOR::L => write!(f, "XOR L")
        }
    }
}

impl XOR {
    pub fn new(opcode: u8) -> XOR {
        match opcode {
            0xAF => XOR::A,
            0xA8 => XOR::B,
            0xA9 => XOR::C,
            0xAA => XOR::D,
            0xAB => XOR::E,
            0xAC => XOR::H,
            0xAD => XOR::L,
            // 0xAE => XOR::HL,
            _ => panic!()
        }
    }
}

impl OpCode for XOR {
    fn exec(&self, gb: &mut GameBoy) {
        let param = match self {
            XOR::A => gb.register.a,
            XOR::B => gb.register.b,
            XOR::C => gb.register.c,
            XOR::D => gb.register.d,
            XOR::E => gb.register.e,
            XOR::H => gb.register.h,
            XOR::L => gb.register.l
        };
        let result = gb.register.a ^ param;
        gb.register.a = result;
        gb.register.f = if result == 0 {
            Flags::Z
        } else {
            Flags::empty()
        };
        gb.register.pc += 1;
    }
}