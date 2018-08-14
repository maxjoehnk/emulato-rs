extern crate byteorder;
#[macro_use]
extern crate bitflags;

mod opcodes;
mod gameboy;
mod register;

use opcodes::opcode::OpCode;

fn main() {
    let mut gb = gameboy::GameBoy::new();

    println!("{:?}", gb);
    while let Some(cmd) = gb.next() {
        println!("executing {:?}", cmd);
        cmd.exec(&mut gb);
        println!("{:?}", gb);
    }
}
