use gameboy::GameBoy;
use cpu::Instruction;
use std::fmt;
use cpu::register::{Flags, Register8, Register16};

pub struct IncrementRegister(Register8);

impl IncrementRegister {
    pub fn new(opcode: u8) -> IncrementRegister {
        let register = match opcode {
            0x04 => Register8::B,
            0x0C => Register8::C,
            0x14 => Register8::D,
            0x1C => Register8::E,
            0x24 => Register8::H,
            0x2C => Register8::L,
            0x3C => Register8::A,
            _ => unreachable!()
        };
        IncrementRegister(register)
    }
}

impl fmt::Debug for IncrementRegister {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "INC {:?}", self.0)
    }
}

impl Instruction for IncrementRegister {
    fn exec(&self, gb: &mut GameBoy) {
        {
            let mut value = gb.register.get(&self.0);
            value = value.wrapping_add(1);
            gb.register.write_8bit_register(&self.0, value);

            let half_carry = false; // TODO: get half carry bit
            let result = value == 0u8;
            gb.register.f.set(Flags::Z, result);
            gb.register.f.remove(Flags::N);
            gb.register.f.set(Flags::H, half_carry);
        }
        pc!(gb);
    }
}

pub struct Increment16BitRegister(pub Register16);

impl fmt::Debug for Increment16BitRegister {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "INC {:?}", self.0)
    }
}

impl Instruction for Increment16BitRegister {
    fn exec(&self, gb: &mut GameBoy) {
        let mut value = gb.register.read_16bit_register(&self.0);
        value = value.wrapping_add(1);
        gb.register.write_16bit_register(&self.0, value);
        pc!(gb);
    }
}

#[cfg(test)]
mod tests {
    mod one_byte {
        use super::super::*;
        use cpu::register::*;
        use gameboy::GameBoy;

        #[test]
        fn it_should_increment_the_value_of_register_a() {
            let mut gb = GameBoy::new();
            let instruction = IncrementRegister(Register8::A);
            instruction.exec(&mut gb);
            assert_eq!(gb.register.a, 0x01);
        }

        #[test]
        fn it_should_increment_the_value_of_register_b() {
            let mut gb = GameBoy::new();
            let instruction = IncrementRegister(Register8::B);
            instruction.exec(&mut gb);
            assert_eq!(gb.register.b, 0x01);
        }

        #[test]
        fn it_should_increment_the_value_of_register_c() {
            let mut gb = GameBoy::new();
            let instruction = IncrementRegister(Register8::C);
            instruction.exec(&mut gb);
            assert_eq!(gb.register.c, 0x01);
        }

        #[test]
        fn it_should_increment_the_value_of_register_d() {
            let mut gb = GameBoy::new();
            let instruction = IncrementRegister(Register8::D);
            instruction.exec(&mut gb);
            assert_eq!(gb.register.d, 0x01);
        }

        #[test]
        fn it_should_increment_the_value_of_register_e() {
            let mut gb = GameBoy::new();
            let instruction = IncrementRegister(Register8::E);
            instruction.exec(&mut gb);
            assert_eq!(gb.register.e, 0x01);
        }

        #[test]
        fn it_should_increment_the_value_of_register_h() {
            let mut gb = GameBoy::new();
            let instruction = IncrementRegister(Register8::H);
            instruction.exec(&mut gb);
            assert_eq!(gb.register.h, 0x01);
        }

        #[test]
        fn it_should_increment_the_value_of_register_l() {
            let mut gb = GameBoy::new();
            let instruction = IncrementRegister(Register8::L);
            instruction.exec(&mut gb);
            assert_eq!(gb.register.l, 0x01);
        }

        #[test]
        fn it_should_overflow_while_incrementing() {
            let mut gb = GameBoy::new();
            gb.register.a = 0xff;
            let instruction = IncrementRegister(Register8::A);
            instruction.exec(&mut gb);
            assert_eq!(gb.register.a, 0x00);
        }
    }

    mod two_bytes {
        use super::super::*;
        use cpu::register::*;
        use gameboy::GameBoy;

        #[test]
        fn it_should_increment_the_value_of_register_pair_af() {
            let mut gb = GameBoy::new();
            let instruction = Increment16BitRegister(Register16::AF);
            instruction.exec(&mut gb);
            assert_eq!(gb.register.a, 0x01);
            assert_eq!(gb.register.f.bits(), 0x00);
        }

        #[test]
        fn it_should_increment_the_value_of_register_pair_bc() {
            let mut gb = GameBoy::new();
            let instruction = Increment16BitRegister(Register16::BC);
            instruction.exec(&mut gb);
            assert_eq!(gb.register.b, 0x01);
            assert_eq!(gb.register.c, 0x00);
        }

        #[test]
        fn it_should_increment_the_value_of_register_pair_de() {
            let mut gb = GameBoy::new();
            let instruction = Increment16BitRegister(Register16::DE);
            instruction.exec(&mut gb);
            assert_eq!(gb.register.d, 0x01);
            assert_eq!(gb.register.e, 0x00);
        }

        #[test]
        fn it_should_increment_the_value_of_register_pair_hl() {
            let mut gb = GameBoy::new();
            let instruction = Increment16BitRegister(Register16::HL);
            instruction.exec(&mut gb);
            assert_eq!(gb.register.h, 0x01);
            assert_eq!(gb.register.l, 0x00);
        }

        #[test]
        fn it_should_increment_the_value_of_register_pair_sp() {
            let mut gb = GameBoy::new();
            let instruction = Increment16BitRegister(Register16::SP);
            instruction.exec(&mut gb);
            assert_eq!(gb.register.sp, 0x0001);
        }

        #[test]
        fn it_should_overflow_while_incrementing() {
            let mut gb = GameBoy::new();
            let instruction = Increment16BitRegister(Register16::SP);
            gb.register.sp = 0xffff;
            instruction.exec(&mut gb);
            assert_eq!(gb.register.sp, 0x0000);
        }
    }
}