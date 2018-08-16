use gameboy::GameBoy;
use cpu::Instruction;
use std::fmt;
use cpu::register::{Register8, Flags};

pub struct RotateRegisterLeft(pub Register8);

impl fmt::Debug for RotateRegisterLeft {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RL {:?}", self.0)
    }
}

impl Instruction for RotateRegisterLeft {
    fn exec(&self, gb: &mut GameBoy) {
        rotate_left(gb, &self.0);
        pc!(gb, 2);
    }
}
pub struct RotateRegisterALeft;

impl fmt::Debug for RotateRegisterALeft {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RLA")
    }
}

impl Instruction for RotateRegisterALeft {
    fn exec(&self, gb: &mut GameBoy) {
        rotate_left(gb, &Register8::A);
        pc!(gb);
    }
}

fn rotate_left(gb: &mut GameBoy, target_register: &Register8) {
    let (result, carry) = {
        let bit0 = if gb.register.f.contains(Flags::C) {
            0b0000_0001
        }else {
            0b0000_0000
        };
        let register = gb.register.get_mut(target_register);
        let carry = *register & 0b1000_0000 > 0;
        if *register == 0b1111_1111 {
            *register = 0b1111_1110;
        }else {
            *register <<= 1;
        }
        *register += bit0;
        let result = *register == 0;
        (result, carry)
    };
    gb.register.f.set(Flags::Z, result);
    gb.register.f.remove(Flags::N);
    gb.register.f.remove(Flags::H);
    gb.register.f.set(Flags::C, carry);
}