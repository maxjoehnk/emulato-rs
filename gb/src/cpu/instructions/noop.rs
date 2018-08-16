use gameboy::GameBoy;
use cpu::Instruction;

#[derive(Debug)]
pub struct NoOp;
impl Instruction for NoOp {
    fn exec(&self, gb: &mut GameBoy) {
        pc!(gb);
    }
}