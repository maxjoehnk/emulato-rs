use byteorder::{LittleEndian, ByteOrder, ReadBytesExt};
use register::TargetRegister;
use register::{Register8, Register16};

macro_rules! cmd {
    ($cmd:expr) => (Some(Box::new($cmd)));
}

macro_rules! u8 {
    ($rom:expr) => ($rom[0])
}

macro_rules! u16 {
    ($rom:expr) => (read_u16($rom))
}

macro_rules! r8 {
    ($register:ident) => (Register8::$register);
}

pub mod opcode;
mod bit;
mod dec;
mod inc;
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
        /* NOP */
        0x00 =>
            cmd!(noop::NoOp),
        /* LD BC,nn */
        0x01 =>
            cmd!(load::Load16Bit::BC(u16!(rom))),
        /* LD DE,nn */
        0x11 =>
            cmd!(load::Load16Bit::DE(u16!(rom))),
        /* LD HL,nn */
        0x21 =>
            cmd!(load::Load16Bit::HL(u16!(rom))),
        /* LD SP,nn */
        0x31 =>
            cmd!(load::Load16Bit::SP(u16!(rom))),
        /* LD (r),A */
        0x02 | 0x12 | 0x77 =>
            cmd!(load::LoadIntoRegisterRamFromRegisterA::new(opcode)),
        /* INC n */
        0x04 | 0x0C | 0x14 | 0x1C | 0x24 | 0x2C | 0x3C =>
            cmd!(inc::IncrementRegister::new(opcode)),
        /* DEC A */
        0x3D =>
            cmd!(dec::DecrementRegister(r8!(A))),
        /* DEC B */
        0x05 =>
            cmd!(dec::DecrementRegister(r8!(B))),
        /* DEC C */
        0x0D =>
            cmd!(dec::DecrementRegister(r8!(C))),
        /* DEC D */
        0x15 =>
            cmd!(dec::DecrementRegister(r8!(D))),
        /* DEC E */
        0x1D =>
            cmd!(dec::DecrementRegister(r8!(E))),
        /* DEC H */
        0x25 =>
            cmd!(dec::DecrementRegister(r8!(H))),
        /* DEC L */
        0x2D =>
            cmd!(dec::DecrementRegister(r8!(L))),
        /* */
        0x06 | 0x0E | 0x16 | 0x1E | 0x26 | 0x2E | 0x3E =>
            cmd!(load::Load8Bit::new(opcode, u8!(rom))),
        /* LD A,(BC) */
        0x0A =>
            cmd!(load::LoadRegisterRamIntoRegisterA::bc()),
        /* LD A,(DE) */
        0x1A =>
            cmd!(load::LoadRegisterRamIntoRegisterA::de()),
        /* JR NZ,n */
        0x20 =>
            cmd!(jump::Jump::nz(read_i8(rom))),
        /* LD (HL-),A */
        0x32 =>
            cmd!(load::LoadDecrementHLA),
        /* LD A,(HL) */
        0x7E =>
            cmd!(load::LoadRegisterRamIntoRegisterA::hl()),
        /* */
        0xAF | 0xA8 | 0xA9 | 0xAA | 0xAB | 0xAC | 0xAD =>
            cmd!(xor::XOR::new(opcode)),
        /* CB */
        0xCB => parse_prefix_command(rom),
        /* LDH (n),A */
        0xE0 =>
            cmd!(load::LoadRegisterAIntoZeroPageRam(rom[0])),
        /* LD (nn),A */
        0xE2 =>
            cmd!(load::LoadRamFromRegisterA),
        /* */
        0xEA =>
            cmd!(load::LoadIntoImmediateRamFromRegisterA(u16!(rom))),
        /* LD A,(n) */
        0xFA =>
            cmd!(load::LoadImmediateRamIntoRegisterA(u16!(rom))),
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
                0x78 => Register8::B,
                0x79 => Register8::C,
                0x7A => Register8::D,
                0x7B => Register8::E,
                0x7C => Register8::H,
                0x7D => Register8::L,
                // 0x7E => TargetRegister::HL,
                0x7F => Register8::A,
                _ => unreachable!()
            };
            cmd!(bit::BIT {
                bit,
                register
            })
        },
        _ => None

    }

}