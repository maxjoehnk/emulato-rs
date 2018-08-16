use gameboy::GameBoy;
use cpu::Instruction;
use std::fmt;
use cpu::register::Flags;

#[derive(Debug)]
enum JumpMode {
    NZ,
    Z,
    NC,
    C
}

/**
 * JR cc,n
 *
 * n = one byte signed immediate value
 *
 * cc = NZ, Jump if Z flag is reset
 * cc = Z, Jump if Z flag is set.
 * cc = NC, Jump if C flag is reset.
 * cc = C, Jump if C flag is set.
 */
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

impl Instruction for Jump {
    fn exec(&self, gb: &mut GameBoy) {
        let result = match self.mode {
            JumpMode::NZ => !gb.register.f.contains(Flags::Z),
            JumpMode::Z => gb.register.f.contains(Flags::Z),
            JumpMode::NC => !gb.register.f.contains(Flags::C),
            JumpMode::C => gb.register.f.contains(Flags::C),
        };
        pc!(gb, 2);
        if result {
            gb.register.pc = (gb.register.pc as i16 + self.target as i16) as u16;
        }
    }
}