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
pub enum Register8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L
}

#[derive(Debug)]
pub enum Register16 {
    SP,
    AF,
    BC,
    DE,
    HL
}

#[derive(Debug, Copy, Clone)]
pub enum RegisterPair {
    AF,
    BC,
    DE,
    HL
}

impl From<RegisterPair> for Register16 {
    fn from(pair: RegisterPair) -> Self {
        match pair {
            RegisterPair::AF => Register16::AF,
            RegisterPair::BC => Register16::BC,
            RegisterPair::DE => Register16::DE,
            RegisterPair::HL => Register16::HL,
        }
    }
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
    pub fn pair(&self, pair: &RegisterPair) -> u16 {
        match pair {
            RegisterPair::AF => self.read_af(),
            RegisterPair::BC => self.read_bc(),
            RegisterPair::DE => self.read_de(),
            RegisterPair::HL => self.read_hl(),
        }
    }

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

    pub fn read_8bit_register(&self, target: &Register8) -> u8 {
        match target {
            Register8::A => self.a,
            Register8::B => self.b,
            Register8::C => self.c,
            Register8::D => self.d,
            Register8::E => self.e,
            Register8::H => self.h,
            Register8::L => self.l
        }
    }

    pub fn write_8bit_register(&mut self, target: &Register8, data: u8) {
        let target = match target {
            Register8::A => &mut self.a,
            Register8::B => &mut self.b,
            Register8::C => &mut self.c,
            Register8::D => &mut self.d,
            Register8::E => &mut self.e,
            Register8::H => &mut self.h,
            Register8::L => &mut self.l
        };
        *target = data;
    }

    pub fn read_16bit_register(&self, target: &Register16) -> u16 {
        match target {
            Register16::SP => self.sp,
            Register16::AF => self.read_af(),
            Register16::BC => self.read_bc(),
            Register16::DE => self.read_de(),
            Register16::HL => self.read_hl()
        }
    }

    pub fn write_16bit_register(&mut self, target: &Register16, data: u16) {
        match target {
            Register16::SP => self.sp = data,
            Register16::AF => self.write_af(data),
            Register16::BC => self.write_bc(data),
            Register16::DE => self.write_de(data),
            Register16::HL => self.write_hl(data)
        }
    }

    pub fn get(&self, target: &Register8) -> u8 {
        self.read_8bit_register(target)
    }

    pub fn get_mut(&mut self, target: &Register8) -> &mut u8 {
        match target {
            Register8::A => &mut self.a,
            Register8::B => &mut self.b,
            Register8::C => &mut self.c,
            Register8::D => &mut self.d,
            Register8::E => &mut self.e,
            Register8::H => &mut self.h,
            Register8::L => &mut self.l
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