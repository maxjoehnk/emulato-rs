extern crate byteorder;
#[macro_use]
extern crate bitflags;

#[macro_use]
mod macros;
mod register;
mod gameboy;
mod opcodes;

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
