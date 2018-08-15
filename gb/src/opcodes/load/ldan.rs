use gameboy::GameBoy;
use opcodes::opcode::OpCode;
use std::fmt;
use register::{Register8, Register16};


/**
 * LD A,n
 */
pub struct LoadRegisterIntoRegisterA(pub Register8);

impl fmt::Debug for LoadRegisterIntoRegisterA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LD A,{:?}", self.0)
    }
}

impl OpCode for LoadRegisterIntoRegisterA {
    fn exec(&self, gb: &mut GameBoy) {
        let value = gb.register.get(&self.0);
        gb.register.a = value;
        pc!(gb);
    }
}

/**
 * LD A,n
 */
pub struct LoadRegisterRamIntoRegisterA(Register16);

impl LoadRegisterRamIntoRegisterA {
    pub fn bc() -> LoadRegisterRamIntoRegisterA {
        LoadRegisterRamIntoRegisterA(Register16::BC)
    }

    pub fn de() -> LoadRegisterRamIntoRegisterA {
        LoadRegisterRamIntoRegisterA(Register16::DE)
    }

    pub fn hl() -> LoadRegisterRamIntoRegisterA {
        LoadRegisterRamIntoRegisterA(Register16::HL)
    }
}

impl fmt::Debug for LoadRegisterRamIntoRegisterA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LD A,({:?})", self.0)
    }
}

impl OpCode for LoadRegisterRamIntoRegisterA {
    fn exec(&self, gb: &mut GameBoy) {
        let location = gb.register.read_16bit_register(&self.0) as usize;
        gb.register.a = gb.ram[location];
        pc!(gb);
    }
}

/**
 * LD A,(nn)
 *
 * nn = two byte immediate value
 */
pub struct LoadImmediateRamIntoRegisterA(pub u16);

impl fmt::Debug for LoadImmediateRamIntoRegisterA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LD A,(0x{:x?})", self.0)
    }
}

impl OpCode for LoadImmediateRamIntoRegisterA {
    fn exec(&self, gb: &mut GameBoy) {
        let location = self.0 as usize;
        gb.register.a = gb.ram[location];
        pc!(gb, 2);
    }
}