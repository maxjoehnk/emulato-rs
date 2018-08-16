use gameboy::GameBoy;
use cpu::Instruction;
use cpu::register::{Flags, Register8};
use std::fmt;

pub struct DecrementRegister(pub Register8);

impl fmt::Debug for DecrementRegister {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DEC {:?}", self.0)
    }
}

impl Instruction for DecrementRegister {
    fn exec(&self, gb: &mut GameBoy) {
        // Wrapping Decrement
        let register = gb.register.get(&self.0);
        let after = register.wrapping_sub(1);
        gb.register.write_8bit_register(&self.0, after);

        // Update Flags
        let half_carry = false; // TODO: get half carry bit
        let result = after == 0u8;
        gb.register.f.set(Flags::Z, result);
        gb.register.f.set(Flags::N, true);
        gb.register.f.set(Flags::H, half_carry);

        // Increment Program Counter
        pc!(gb);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cpu::register::*;
    use gameboy::GameBoy;

    #[test]
    fn it_should_increase_the_pc() {
        let mut gb = GameBoy::new();
        let instruction = DecrementRegister(Register8::A);
        instruction.exec(&mut gb);
        assert_eq!(gb.register.pc, 0x01);
    }

    mod one_byte {
        use super::super::*;
        use cpu::register::*;
        use gameboy::GameBoy;

        #[test]
        fn it_should_decrement_the_value_of_register_a() {
            let mut gb = GameBoy::new();
            let instruction = DecrementRegister(Register8::A);
            gb.register.a = 0x01;
            instruction.exec(&mut gb);
            assert_eq!(gb.register.a, 0x00);
        }

        #[test]
        fn it_should_decrement_the_value_of_register_b() {
            let mut gb = GameBoy::new();
            let instruction = DecrementRegister(Register8::B);
            gb.register.b = 0x01;
            instruction.exec(&mut gb);
            assert_eq!(gb.register.b, 0x00);
        }

        #[test]
        fn it_should_decrement_the_value_of_register_c() {
            let mut gb = GameBoy::new();
            let instruction = DecrementRegister(Register8::C);
            gb.register.c = 0x01;
            instruction.exec(&mut gb);
            assert_eq!(gb.register.c, 0x00);
        }

        #[test]
        fn it_should_decrement_the_value_of_register_d() {
            let mut gb = GameBoy::new();
            let instruction = DecrementRegister(Register8::D);
            gb.register.d = 0x01;
            instruction.exec(&mut gb);
            assert_eq!(gb.register.d, 0x00);
        }

        #[test]
        fn it_should_decrement_the_value_of_register_e() {
            let mut gb = GameBoy::new();
            let instruction = DecrementRegister(Register8::E);
            gb.register.e = 0x01;
            instruction.exec(&mut gb);
            assert_eq!(gb.register.e, 0x00);
        }

        #[test]
        fn it_should_decrement_the_value_of_register_h() {
            let mut gb = GameBoy::new();
            let instruction = DecrementRegister(Register8::H);
            gb.register.h = 0x01;
            instruction.exec(&mut gb);
            assert_eq!(gb.register.h, 0x00);
        }

        #[test]
        fn it_should_decrement_the_value_of_register_l() {
            let mut gb = GameBoy::new();
            let instruction = DecrementRegister(Register8::L);
            gb.register.l = 0x01;
            instruction.exec(&mut gb);
            assert_eq!(gb.register.l, 0x00);
        }

        #[test]
        fn it_should_overflow_while_decrementing() {
            let mut gb = GameBoy::new();
            let instruction = DecrementRegister(Register8::A);
            instruction.exec(&mut gb);
            assert_eq!(gb.register.a, 0xff);
        }
    }
}