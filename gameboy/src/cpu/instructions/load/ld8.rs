use cpu::Instruction;
use gameboy::GameBoy;
use std::fmt;
use cpu::register::Register8;

pub struct Load8Bit {
    register: Register8,
    data: u8
}

impl Load8Bit {
    pub fn new(opcode: u8, data: u8) -> Load8Bit {
        let register = match opcode {
            0x06 => Register8::B,
            0x0E => Register8::C,
            0x16 => Register8::D,
            0x1E => Register8::E,
            0x26 => Register8::H,
            0x2E => Register8::L,
            0x3E => Register8::A,
            _ => unreachable!()
        };
        Load8Bit {
            register,
            data
        }
    }
}

impl Instruction for Load8Bit {
    fn exec(&self, gb: &mut GameBoy) {
        gb.register.write_8bit_register(&self.register, self.data);
        pc!(gb, 2)
    }
}

impl fmt::Debug for Load8Bit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LD {:?},0x{:x?}", self.register, self.data)
    }
}