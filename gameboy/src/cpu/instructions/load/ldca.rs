use gameboy::GameBoy;
use cpu::Instruction;
use std::fmt;

pub struct LoadRamFromRegisterA;

impl fmt::Debug for LoadRamFromRegisterA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LD (C),A")
    }
}

impl Instruction for LoadRamFromRegisterA {
    fn exec(&self, gb: &mut GameBoy) {
        let c = gb.register.c as u16;
        let a = gb.register.a;
        let index = (0xff00 + c) as usize;
        gb.ram[index] = a;
        pc!(gb);
    }
}