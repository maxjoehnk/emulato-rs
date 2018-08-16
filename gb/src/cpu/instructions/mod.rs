use byteorder::{LittleEndian, ByteOrder};
use cpu::register::{Register8, Register16, RegisterPair};
use cpu::Instruction;

mod alu;
mod call;
mod compare;
mod dec;
mod inc;
mod jump;
mod noop;
mod load;
mod xor;
mod push;
mod ret;
mod pop;

pub fn parse_command(opcode: u8, rom: &[u8]) -> Option<Box<dyn Instruction>> {
    match opcode {
        /* NOP */
        //0x00 =>
        //    cmd!(noop::NoOp),
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
        /* RLA */
        0x17 =>
            cmd!(alu::RotateRegisterALeft),
        /* INC BC */
        0x03 =>
            cmd!(inc::Increment16BitRegister(r16!(BC))),
        /* INC DE */
        0x13 =>
            cmd!(inc::Increment16BitRegister(r16!(DE))),
        /* INC HL */
        0x23 =>
            cmd!(inc::Increment16BitRegister(r16!(HL))),
        /* INC SP */
        0x33 =>
            cmd!(inc::Increment16BitRegister(r16!(SP))),
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
        /* LD A,A */
        0x7F =>
            cmd!(load::LoadRegisterIntoRegisterA(r8!(A))),
        /* LD A,B */
        0x78 =>
            cmd!(load::LoadRegisterIntoRegisterA(r8!(B))),
        /* LD A,C */
        0x79 =>
            cmd!(load::LoadRegisterIntoRegisterA(r8!(C))),
        /* LD A,D */
        0x7A =>
            cmd!(load::LoadRegisterIntoRegisterA(r8!(D))),
        /* LD A,E */
        0x7B =>
            cmd!(load::LoadRegisterIntoRegisterA(r8!(E))),
        /* LD A,H */
        0x7C =>
            cmd!(load::LoadRegisterIntoRegisterA(r8!(H))),
        /* LD A,L */
        0x7D =>
            cmd!(load::LoadRegisterIntoRegisterA(r8!(L))),
        /* LD A,(BC) */
        0x0A =>
            cmd!(load::LoadRegisterRamIntoRegisterA(rp!(BC))),
        /* LD A,(DE) */
        0x1A =>
            cmd!(load::LoadRegisterRamIntoRegisterA(rp!(DE))),
        /* LD A,(HL) */
        0x7E =>
            cmd!(load::LoadRegisterRamIntoRegisterA(rp!(HL))),
        /* JR NZ,n */
        0x20 =>
            cmd!(jump::Jump::nz(i8!(rom))),
        /* LD (HL+),A */
        0x22 =>
            cmd!(load::LoadIncrementHLA),
        /* LD (HL-),A */
        0x32 =>
            cmd!(load::LoadDecrementHLA),
        /* */
        0xAF | 0xA8 | 0xA9 | 0xAA | 0xAB | 0xAC | 0xAD =>
            cmd!(xor::XOR::new(opcode)),
        /* LD B,A */
        0x47 =>
            cmd!(load::LoadIntoRegisterFromRegisterA(r8!(B))),
        /* LD C,A */
        0x4F =>
            cmd!(load::LoadIntoRegisterFromRegisterA(r8!(C))),
        /* CB */
        0xCB => parse_prefix_command(rom),
        /* CALL nn */
        0xCD => cmd!(call::Call(u16!(rom))),
        /* RET */
        0xC9 => cmd!(ret::Return),
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
        /* PUSH AF */
        0xF5 =>
            cmd!(push::Push(rp!(AF))),
        /* PUSH BC */
        0xC5 =>
            cmd!(push::Push(rp!(BC))),
        /* PUSH DE */
        0xD5 =>
            cmd!(push::Push(rp!(DE))),
        /* PUSH HL */
        0xE5 =>
            cmd!(push::Push(rp!(HL))),
        /* POP AF */
        0xF1 =>
            cmd!(pop::Pop(rp!(AF))),
        /* POP BC */
        0xC1 =>
            cmd!(pop::Pop(rp!(BC))),
        /* POP DE */
        0xD1 =>
            cmd!(pop::Pop(rp!(DE))),
        /* POP HL */
        0xE1 =>
            cmd!(pop::Pop(rp!(HL))),
        /* CP # */
        0xFE =>
            cmd!(compare::CompareImmediate(u8!(rom))),
        _ => {
            println!("Unknown OpCode {:#X?}", opcode);
            None
        }
    }
}

fn parse_prefix_command(rom: &[u8]) -> Option<Box<dyn Instruction>> {
    let opcode = rom[0];
    match opcode {
        /* RLC r */
        0x00...0x07 => unimplemented!("RLC r"),
        /* RRC r */
        0x08...0x0F => unimplemented!("RRC r"),
        /* RL B */
        0x10 => cmd!(alu::RotateRegisterLeft(r8!(B))),
        /* RL C */
        0x11 => cmd!(alu::RotateRegisterLeft(r8!(C))),
        /* RL D */
        0x12 => cmd!(alu::RotateRegisterLeft(r8!(D))),
        /* RL E */
        0x13 => cmd!(alu::RotateRegisterLeft(r8!(E))),
        /* RL H */
        0x14 => cmd!(alu::RotateRegisterLeft(r8!(H))),
        /* RL L */
        0x15 => cmd!(alu::RotateRegisterLeft(r8!(L))),
        /* RL A */
        0x17 => cmd!(alu::RotateRegisterLeft(r8!(A))),
        /* RL (HL) */
        0x16 => unimplemented!("RL (HL)"),
        /* RR r */
        0x17...0x1F => unimplemented!("RR r"),
        /* SLA r */
        0x20...0x27 => unimplemented!("SLA r"),
        /* SRA r */
        0x27...0x2F => unimplemented!("SRA r"),
        /* BIT 0,r */
        0x40...0x47 => unimplemented!("BIT 0,r"),
        /* BIT 1,r */
        0x47...0x4F => unimplemented!("BIT 1,r"),
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
            cmd!(alu::Bit {
                bit,
                register
            })
        },
        _ => None

    }

}