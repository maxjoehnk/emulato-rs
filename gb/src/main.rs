extern crate byteorder;
#[macro_use]
extern crate bitflags;
extern crate tui;

#[macro_use]
mod macros;
mod register;
mod gameboy;
mod opcodes;
mod gui;

use opcodes::opcode::OpCode;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let useGui = args.len() > 1 && args[1] == "gui";

    let mut gb = gameboy::GameBoy::new();
    let mut instructions = Vec::new();
    let mut tui = if useGui {
        Some(gui::terminal::build())
    }else {
        None
    };

    while let Some(cmd) = gb.next() {
        if let Some(ref mut tui) = tui {
            tui.draw(&gb, &instructions);
        }else {
            println!("{:?}", gb);
            println!("{:?}", cmd);
        }
        instructions.push(format!("{:?}", cmd));
        cmd.exec(&mut gb);
    }
}
