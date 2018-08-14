use std::fmt;
use gameboy::GameBoy;

pub trait OpCode: fmt::Debug {
    fn exec(&self, gb: &mut GameBoy);
}