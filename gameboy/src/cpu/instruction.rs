use std::fmt;
use gameboy::GameBoy;

pub trait Instruction: fmt::Debug {
    fn exec(&self, gb: &mut GameBoy);
}