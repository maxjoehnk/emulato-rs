use gameboy::GameBoy;
use cpu::Instruction;
use std::fmt;
use cpu::register::RegisterPair;

pub struct Push(pub RegisterPair);

impl fmt::Debug for Push {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PUSH {:?}", self.0)
    }
}

impl Instruction for Push {
    fn exec(&self, gb: &mut GameBoy) {
        let register = gb.register.pair(&self.0);
        gb.push_to_stack(register);
        pc!(gb);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use gameboy::GameBoy;
    use cpu::register::RegisterPair;

    #[test]
    fn write_16_bit_register_into_stack() {
        let mut gb = GameBoy::new();
        gb.register.sp = 0xfffe;
        gb.register.b = 0x12;
        gb.register.c = 0x34;

        let instruction = Push(RegisterPair::BC);
        instruction.exec(&mut gb);

        assert_eq!(gb.ram[0xfffe], 0x12);
        assert_eq!(gb.ram[0xfffd], 0x34);
    }

    #[test]
    fn should_increase_the_pc() {
        let mut gb = GameBoy::new();
        gb.register.sp = 0xfffc;

        let instruction = Push(RegisterPair::BC);
        instruction.exec(&mut gb);

        assert_eq!(gb.register.pc, 0x01);
    }
}