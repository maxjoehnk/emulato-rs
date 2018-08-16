use std::fmt;
use cpu::register::Register;
use cpu::{instructions, Instruction};
use byteorder::{LittleEndian, WriteBytesExt, ByteOrder};

pub struct GameBoy {
    pub register: Register,
    pub ram: [u8; 0xffff]
}

impl fmt::Debug for GameBoy {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "GameBoy {{ registers: {:?} }}", self.register)
    }
}

impl GameBoy {
    pub fn new() -> GameBoy {
        let mut gb = GameBoy {
            register: Register::default(),
            ram: [0; 0xffff]
        };
        let firmware = include_bytes!("../assets/DMG_ROM.bin");

        for (i, byte) in firmware.iter().enumerate() {
            gb.ram[i] = *byte;
        }

        return gb;
    }

    pub fn ram_mut(&mut self) -> &mut [u8; 0xffff] {
        &mut self.ram
    }

    pub fn push_to_stack(&mut self, addr: u16) {
        let mut data = vec![];
        data.write_u16::<LittleEndian>(addr).unwrap();
        let sp = self.register.sp as usize;
        self.ram[sp] = data[0];
        self.ram[sp - 1] = data[1];

        self.register.sp -= 2;
    }

    pub fn pop_from_stack(&mut self) -> u16 {
        let sp = self.register.sp as usize;
        let b1 = self.ram[sp + 2];
        let b2 = self.ram[sp + 1];

        self.register.sp += 2;

        LittleEndian::read_u16(&[b1, b2])
    }
}

impl Iterator for GameBoy {
    type Item = Box<dyn Instruction>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let pc = self.register.pc as usize;
        if pc >= 65535 {
            println!("Reached end of Ram. Exiting...");
            return None;
        }

        let opcode = self.ram[pc];

        instructions::parse_command(opcode, &self.ram[pc + 1..])
    }
}