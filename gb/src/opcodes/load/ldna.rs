use gameboy::GameBoy;
use opcodes::opcode::OpCode;
use std::fmt;
use register::{Register8, Register16};

pub struct LoadIntoRegisterFromRegisterA(pub Register8);

impl fmt::Debug for LoadIntoRegisterFromRegisterA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LD {:?},A", self.0)
    }
}

impl OpCode for LoadIntoRegisterFromRegisterA {
    fn exec(&self, gb: &mut GameBoy) {
        let a = gb.register.a;
        {
            let register = gb.register.get_mut(&self.0);
            *register = a;
        }
        pc!(gb);
    }
}

pub struct LoadIntoRegisterRamFromRegisterA(Register16);

impl LoadIntoRegisterRamFromRegisterA {
    pub fn new(opcode: u8) -> LoadIntoRegisterRamFromRegisterA {
        let register = match opcode {
            0x02 => Register16::BC,
            0x12 => Register16::DE,
            0x77 => Register16::HL,
            _ => unreachable!()
        };
        LoadIntoRegisterRamFromRegisterA(register)
    }
}

impl fmt::Debug for LoadIntoRegisterRamFromRegisterA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LD ({:?}),A", self.0)
    }
}

impl OpCode for LoadIntoRegisterRamFromRegisterA {
    fn exec(&self, gb: &mut GameBoy) {
        let a = gb.register.a;
        {
            let location = gb.register.read_16bit_register(&self.0) as usize;
            let ram = gb.ram_mut();
            ram[location] = a;
        }
        pc!(gb);
    }
}
pub struct LoadIntoImmediateRamFromRegisterA(pub u16);

impl fmt::Debug for LoadIntoImmediateRamFromRegisterA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LD (0x{:x?}),A", self.0)
    }
}

impl OpCode for LoadIntoImmediateRamFromRegisterA {
    fn exec(&self, gb: &mut GameBoy) {
        let a = gb.register.a;
        {
            let location = self.0 as usize;
            let ram = gb.ram_mut();
            ram[location] = a;
        }
        pc!(gb, 2);
    }
}