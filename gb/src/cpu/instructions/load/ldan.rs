use gameboy::GameBoy;
use cpu::Instruction;
use std::fmt;
use cpu::register::{Register8, Register16, RegisterPair};

/**
 * LD A,n
 */
pub struct LoadRegisterIntoRegisterA(pub Register8);

impl fmt::Debug for LoadRegisterIntoRegisterA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LD A,{:?}", self.0)
    }
}

impl Instruction for LoadRegisterIntoRegisterA {
    fn exec(&self, gb: &mut GameBoy) {
        let value = gb.register.get(&self.0);
        gb.register.a = value;
        pc!(gb);
    }
}

/**
 * LD A,n
 */
pub struct LoadRegisterRamIntoRegisterA(pub RegisterPair);

impl fmt::Debug for LoadRegisterRamIntoRegisterA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LD A,({:?})", self.0)
    }
}

impl Instruction for LoadRegisterRamIntoRegisterA {
    fn exec(&self, gb: &mut GameBoy) {
        let location = gb.register.pair(&self.0) as usize;
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

impl Instruction for LoadImmediateRamIntoRegisterA {
    fn exec(&self, gb: &mut GameBoy) {
        let location = self.0 as usize;
        gb.register.a = gb.ram[location];
        pc!(gb, 2);
    }
}