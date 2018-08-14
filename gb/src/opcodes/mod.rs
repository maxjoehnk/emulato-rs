use byteorder::{LittleEndian, ByteOrder, ReadBytesExt};
use register::TargetRegister;

pub mod opcode;
mod bit;
mod jump;
mod noop;
mod load;
mod xor;

fn read_u16(data: &[u8]) -> u16 {
    LittleEndian::read_u16(&data[0..2])
}

fn read_i8(data: &[u8]) -> i8 {
    data[0] as i8
}

pub fn parse_command(opcode: u8, rom: &[u8]) -> Option<Box<dyn opcode::OpCode>> {
    match opcode {
        0x00 => Some(Box::new(noop::NoOp)),
        0x01 | 0x11 | 0x21 | 0x31 => {
            let data = read_u16(rom);
            let cmd = match opcode {
                0x01 => load::Load16Bit::BC(data),
                0x11 => load::Load16Bit::DE(data),
                0x21 => load::Load16Bit::HL(data),
                0x31 => load::Load16Bit::SP(data),
                _ => panic!()
            };
            Some(Box::new(cmd))
        },
        0x20 => Some(Box::new(jump::Jump::nz(read_i8(rom)))),
        0x32 => Some(Box::new(load::LoadDecrementHLA)),
        0xAF | 0xA8 | 0xA9 | 0xAA | 0xAB | 0xAC | 0xAD => {
            Some(Box::new(xor::XOR::new(opcode)))
        },
        0xCB => parse_prefix_command(rom),
        _ => {
            println!("Unknown OP Code 0x{:X?}", opcode);
            None
        }
    }
}

fn parse_prefix_command(rom: &[u8]) -> Option<Box<dyn opcode::OpCode>> {
    let opcode = rom[0];
    match opcode {
        /* RLC r */
        0x00...0x07 => None,
        /* RRC r */
        0x08...0x0F => None,
        /* SLA r */
        0x10...0x17 => None,
        /* SRA r */
        0x17...0x1F => None,
        /* BIT 0,r */
        0x40...0x47 => None,
        /* BIT 1,r */
        0x47...0x4F => None,
        /* BIT 7,r */
        0x78...0x7F => {
            let bit: u8 = 0b1000_0000;
            let register = match opcode {
                0x78 => TargetRegister::B,
                0x79 => TargetRegister::C,
                0x7A => TargetRegister::D,
                0x7B => TargetRegister::E,
                0x7C => TargetRegister::H,
                0x7D => TargetRegister::L,
                0x7E => TargetRegister::HL,
                0x7F => TargetRegister::A,
                _ => panic!("Invalid Opcode {}", opcode)
            };
            Some(Box::new(bit::BIT {
                bit,
                register
            }))
        },
        _ => None

    }

}