
struct GameBoy {
    main_ram: [u8; 4096 * 2],
    vram: [u8; 4096 * 2]
}

impl GameBoy {
    fn new() -> GameBoy {
        GameBoy {
            main_ram: [0; 4096 * 2],
            vram: [0; 4096 * 2]
        }
    }
}

trait OpCode {
    fn exec(&mut gb: GameBoy);
}

type NoOp = ();
impl OpCode for NoOp {
    fn exec(&mut gb: GameBoy) {}
}

enum Load8Bit {
    Bn(u8)
}
impl OpCode for Load8Bit {
    fn exec(&mut gb: GameBoy) {}
}

enum Load16Bit {
    SP([u8; 2])
}
impl OpCode for Load16Bit {
    fn exec(&mut gb: GameBoy) {}
}

fn parse_command<T>(opcode: &u8, data: &mut T) -> Option<impl OpCode> where T: Iterator {
    match opcode {
        31 => {
            let b1 = data.next()?;
            let b2 = data.next()?;
            let d = [b1, b2];
            Some(Load16Bit::SP(d))
        },
        _ => None
    }
}

fn main() {
    let gb = GameBoy::new();
    let bin = include_bytes!("../DMG_ROM.bin");

    let mut iter = bin.iter();
    while let opcode = iter.next() {
        let cmd = parse_command(opcode, &mut iter);
        print!("{:?}", cmd);
    }
}
