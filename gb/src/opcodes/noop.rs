use gameboy::GameBoy;
use opcodes::opcode::OpCode;

#[derive(Debug)]
pub struct NoOp;
impl OpCode for NoOp {
    fn exec(&self, gb: &mut GameBoy) {
        gb.register.pc += 1;
    }
}