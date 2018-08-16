use gameboy::GameBoy;
use cpu::Instruction;
use std::fmt;

pub struct Call(pub u16);

impl fmt::Debug for Call {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CALL {:#X?}", self.0)
    }
}

impl Instruction for Call {
    fn exec(&self, gb: &mut GameBoy) {
        let next_instruction = gb.register.pc + 3;
        gb.push_to_stack(next_instruction);
        gb.register.pc = self.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gameboy::GameBoy;

    #[test]
    fn it_should_set_the_pc() {
        let mut gb = GameBoy::new();
        gb.register.sp = 0xfffe;
        let instruction = Call(0x1234);
        instruction.exec(&mut gb);
        assert_eq!(gb.register.pc, 0x1234);
    }

    #[test]
    fn it_should_store_the_current_pc() {
        let mut gb = GameBoy::new();
        gb.register.sp = 0xfffe;
        gb.register.pc = 0x0ff0;
        let instruction = Call(0x1234);
        instruction.exec(&mut gb);
        assert_eq!(gb.ram[0xfffe], 0xf3);
        assert_eq!(gb.ram[0xfffd], 0x0f);
    }
}