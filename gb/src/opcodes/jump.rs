use gameboy::GameBoy;
use opcodes::opcode::OpCode;
use register::Flags;
use std::fmt;
use register::TargetRegister;

#[derive(Debug)]
enum JumpMode {
    NZ,
    Z,
    NC,
    C
}

pub struct Jump {
    pub target: i8,
    mode: JumpMode
}

impl Jump {
    pub fn nz(target: i8) -> Jump {
        Jump {
            target,
            mode: JumpMode::NZ
        }
    }
}

impl fmt::Debug for Jump {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "JR {:?}, {}", self.mode, self.target)
    }
}

impl OpCode for Jump {
    fn exec(&self, gb: &mut GameBoy) {
        let result = match self.mode {
            JumpMode::NZ => !gb.register.f.contains(Flags::Z),
            JumpMode::Z => gb.register.f.contains(Flags::Z),
            JumpMode::NC => !gb.register.f.contains(Flags::C),
            JumpMode::C => gb.register.f.contains(Flags::C),
        };
        gb.register.pc += 2;
        if result {
            gb.register.pc = (gb.register.pc as i8 + self.target) as u16;
        }
    }
}