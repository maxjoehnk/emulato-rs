use opcodes::opcode::OpCode;
use gameboy::GameBoy;

#[derive(Debug)]
pub enum Load8Bit {
    Bn(u8)
}
impl OpCode for Load8Bit {
    fn exec(&self, gb: &mut GameBoy) {}
}

#[derive(Debug)]
pub enum Load16Bit {
    BC(u16),
    DE(u16),
    HL(u16),
    SP(u16),
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

#[derive(Debug)]
pub struct LoadDecrementHLA;

impl OpCode for LoadDecrementHLA {
    fn exec(&self, gb: &mut GameBoy) {
        let hl = gb.register.read_hl();
        let a = gb.register.a;
        gb.ram[hl as usize] = a;
        gb.register.write_hl(hl - 1);
        gb.register.pc += 1;
    }
}