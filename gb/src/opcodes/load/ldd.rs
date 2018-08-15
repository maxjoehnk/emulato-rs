use gameboy::GameBoy;
use opcodes::opcode::OpCode;
use std::fmt;

pub struct LoadDecrementHLA;

impl fmt::Debug for LoadDecrementHLA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LD (HL-),A")
    }
}

impl OpCode for LoadDecrementHLA {
    fn exec(&self, gb: &mut GameBoy) {
        let hl = gb.register.read_hl();
        let a = gb.register.a;
        gb.ram[hl as usize] = a;
        gb.register.write_hl(hl - 1);
        pc!(gb);
    }
}