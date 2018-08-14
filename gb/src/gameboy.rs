use std::fmt;
use register::Register;
use opcodes::opcode::OpCode;
use opcodes;

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
}

impl Iterator for GameBoy {
    type Item = Box<dyn OpCode>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let pc = self.register.pc as usize;
        if pc >= 65535 {
            println!("Reached end of Ram. Exiting...");
            return None;
        }
        let opcode = self.ram[pc];

        opcodes::parse_command(opcode, &self.ram[pc + 1..])
    }
}