extern crate byteorder;
#[macro_use]
extern crate bitflags;
extern crate tui;

mod cpu;
mod gameboy;
mod gui;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let use_gui = args.len() > 1 && args[1] == "gui";

    let mut gb = gameboy::GameBoy::new();
    let mut instructions = Vec::new();
    let mut tui = if use_gui {
        Some(gui::terminal::build()?)
    }else {
        println!("{:?}", gb);
        None
    };

    while let Some(instruction) = gb.next() {
        if let Some(ref mut tui) = tui {
            tui.draw(&gb, &instructions)?;
        }else {
            println!("{:?}", instruction);
        }
        instructions.push(format!("{:?}", instruction));
        instruction.exec(&mut gb);
        if !use_gui {
            println!("{:?}", gb);
        }
    }

    Ok(())
}
