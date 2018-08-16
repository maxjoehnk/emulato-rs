use gameboy::GameBoy;
use cpu::Instruction;
use std::fmt;

pub struct LoadIncrementHLA;

impl fmt::Debug for LoadIncrementHLA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LD (HL+),A")
    }
}

impl Instruction for LoadIncrementHLA {
    fn exec(&self, gb: &mut GameBoy) {
        let hl = gb.register.read_hl();
        let a = gb.register.a;
        gb.ram[hl as usize] = a;
        gb.register.write_hl(hl + 1);
        pc!(gb);
    }
}