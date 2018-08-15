use byteorder::{ByteOrder, LittleEndian};
use std::fmt;

bitflags! {
    #[derive(Default)]
    pub struct Flags: u8 {
        const Z = 0b10000000; // Zero Flag
        const N = 0b01000000; // Operation Flag
        const H = 0b00100000; // Half Carry Flag
        const C = 0b00010000; // Carry Flag
    }
}

#[derive(Debug)]
pub enum TargetRegister {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    SP,
    PC,
    AF,
    BC,
    DE,
    HL
}

#[derive(Default)]
pub struct Register {
    pub a: u8, // Accumulator
    pub b: u8, // B
    pub c: u8, // C
    pub d: u8, // D
    pub e: u8, // E
    pub f: Flags, // Flags
    pub h: u8, // H
    pub l: u8, // L
    pub sp: u16, // Stack Pointer
    pub pc: u16, // Program Counter
}

impl Register {
    pub fn read_af(&self) -> u16 {
        LittleEndian::read_u16(&[self.a, self.f.bits])
    }

    pub fn read_bc(&self) -> u16 {
        LittleEndian::read_u16(&[self.b, self.c])
    }

    pub fn read_de(&self) -> u16 {
        LittleEndian::read_u16(&[self.d, self.e])
    }

    pub fn read_hl(&self) -> u16 {
        LittleEndian::read_u16(&[self.h, self.l])
    }

    pub fn write_af(&mut self, af: u16) {
        let mut buf = [0; 2];
        LittleEndian::write_u16(&mut buf, af);
        self.a = buf[0];
        self.f = Flags::from_bits(buf[1]).unwrap(); // TODO: remove unwrap
    }

    pub fn write_bc(&mut self, bc: u16) {
        let mut buf = [0; 2];
        LittleEndian::write_u16(&mut buf, bc);
        self.b = buf[0];
        self.c = buf[1];
    }

    pub fn write_de(&mut self, de: u16) {
        let mut buf = [0; 2];
        LittleEndian::write_u16(&mut buf, de);
        self.d = buf[0];
        self.e = buf[1];
    }

    pub fn write_hl(&mut self, hl: u16) {
        let mut buf = [0; 2];
        LittleEndian::write_u16(&mut buf, hl);
        self.h = buf[0];
        self.l = buf[1];
    }

    pub fn read_target(&self, target: &TargetRegister) -> u8 {
        match target {
            TargetRegister::A => self.a,
            TargetRegister::B => self.b,
            TargetRegister::C => self.c,
            TargetRegister::D => self.d,
            TargetRegister::E => self.e,
            TargetRegister::H => self.h,
            TargetRegister::L => self.l,
            _ => unreachable!()
        }
    }
}

impl fmt::Debug for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Register {{ A: 0x{:x?}, B: 0x{:x?}, C: 0x{:x?}, D: 0x{:x?}, E: 0x{:x?}, F: {:?}, H: 0x{:x?}, L: 0x{:x?}, SP: 0x{:x?}, PC: 0x{:x?}, AF: 0x{:x?}, BC: 0x{:x?}, DE: 0x{:x?}, HL: 0x{:x?}}}",
            self.a,
            self.b,
            self.c,
            self.d,
            self.e,
            self.f,
            self.h,
            self.l,
            self.sp,
            self.pc,
            self.read_af(),
            self.read_bc(),
            self.read_de(),
            self.read_hl()
        )
    }
}