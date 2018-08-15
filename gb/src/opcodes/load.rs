use opcodes::opcode::OpCode;
use gameboy::GameBoy;
use std::fmt;

#[derive(Debug)]
pub enum Load8Bit {
    Bn(u8)
}
impl OpCode for Load8Bit {
    fn exec(&self, gb: &mut GameBoy) {}
}

pub enum Load16Bit {
    BC(u16),
    DE(u16),
    HL(u16),
    SP(u16),
}
impl fmt::Debug for Load16Bit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let register = match self {
            Load16Bit::BC(d) => format!("BC,0x{:x?}", d),
            Load16Bit::DE(d) => format!("DE,0x{:x?}", d),
            Load16Bit::HL(d) => format!("HL,0x{:x?}", d),
            Load16Bit::SP(d) => format!("SP,0x{:x?}", d)
        };
        write!(f, "LD {}", register)
    }
}
impl OpCode for Load16Bit {
    fn exec(&self, gb: &mut GameBoy) {
        match self {
            Load16Bit::BC(bytes) => gb.register.write_bc(*bytes),
            Load16Bit::DE(bytes) => gb.register.write_de(*bytes),
            Load16Bit::HL(bytes) => gb.register.write_hl(*bytes),
            Load16Bit::SP(bytes) => gb.register.sp = *bytes
        }
        gb.register.pc += 3;
    }
}

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