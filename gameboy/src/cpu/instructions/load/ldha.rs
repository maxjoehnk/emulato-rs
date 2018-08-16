use gameboy::GameBoy;
use cpu::Instruction;
use std::fmt;

pub struct LoadRegisterAIntoZeroPageRam(pub u8);

impl fmt::Debug for LoadRegisterAIntoZeroPageRam {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LDH (0x{:x?}),A", self.0)
    }
}

impl Instruction for LoadRegisterAIntoZeroPageRam {
    fn exec(&self, gb: &mut GameBoy) {
        let offset = self.0 as u16;
        let a = gb.register.a;
        let index = (0xff00 + offset) as usize;
        gb.ram[index] = a;
        pc!(gb, 2);
    }
}