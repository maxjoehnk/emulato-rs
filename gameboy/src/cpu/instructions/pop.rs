use gameboy::GameBoy;
use cpu::Instruction;
use std::fmt;
use cpu::register::RegisterPair;

pub struct Pop(pub RegisterPair);

impl fmt::Debug for Pop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "POP {:?}", self.0)
    }
}

impl Instruction for Pop {
    fn exec(&self, gb: &mut GameBoy) {
        let value = gb.pop_from_stack();
        gb.register.write_16bit_register(&self.0.into(), value);
        pc!(gb);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gameboy::GameBoy;
    use cpu::register::RegisterPair;

    #[test]
    fn write_16_bit_stack_value_into_register() {
        let mut gb = GameBoy::new();
        gb.register.sp = 0xfffc;
        gb.ram[0xfffe] = 0x12;
        gb.ram[0xfffd] = 0x34;

        let instruction = Pop(RegisterPair::BC);
        instruction.exec(&mut gb);

        assert_eq!(gb.register.b, 0x12);
        assert_eq!(gb.register.c, 0x34);
    }

    #[test]
    fn should_increase_the_pc() {
        let mut gb = GameBoy::new();
        gb.register.sp = 0xfffc;

        let instruction = Pop(RegisterPair::BC);
        instruction.exec(&mut gb);

        assert_eq!(gb.register.pc, 0x01);
    }
}