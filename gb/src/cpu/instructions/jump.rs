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
pub struct JumpRelative {
    pub target: i8,
    mode: JumpMode
}

impl JumpRelative {
    pub fn nz(target: i8) -> JumpRelative {
        JumpRelative {
            target,
            mode: JumpMode::NZ
        }
    }

    pub fn z(target: i8) -> JumpRelative {
        JumpRelative {
            target,
            mode: JumpMode::Z
        }
    }

    pub fn nc(target: i8) -> JumpRelative {
        JumpRelative {
            target,
            mode: JumpMode::NC
        }
    }

    pub fn c(target: i8) -> JumpRelative {
        JumpRelative {
            target,
            mode: JumpMode::C
        }
    }
}

impl fmt::Debug for JumpRelative {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "JR {:?}, {}", self.mode, self.target)
    }
}

impl Instruction for JumpRelative {
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

#[cfg(test)]
mod tests {
    use super::*;
    use gameboy::GameBoy;
    use cpu::register::Flags;

    #[test]
    fn it_should_jump_when_z_flag_is_reset() {
        let mut gb = GameBoy::new();
        let instruction = JumpRelative::nz(10);
        instruction.exec(&mut gb);
        assert_eq!(gb.register.pc, 12);
    }

    #[test]
    fn it_should_not_jump_when_z_flag_is_set() {
        let mut gb = GameBoy::new();
        gb.register.f.set(Flags::Z, true);
        let instruction = JumpRelative::nz(10);
        instruction.exec(&mut gb);
        assert_eq!(gb.register.pc, 2);
    }

    #[test]
    fn it_should_jump_when_z_flag_is_set() {
        let mut gb = GameBoy::new();
        gb.register.f.set(Flags::Z, true);
        let instruction = JumpRelative::z(10);
        instruction.exec(&mut gb);
        assert_eq!(gb.register.pc, 12);
    }

    #[test]
    fn it_should_not_jump_when_z_flag_is_reset() {
        let mut gb = GameBoy::new();
        let instruction = JumpRelative::z(10);
        instruction.exec(&mut gb);
        assert_eq!(gb.register.pc, 2);
    }

    #[test]
    fn it_should_jump_when_c_flag_is_reset() {
        let mut gb = GameBoy::new();
        let instruction = JumpRelative::nc(10);
        instruction.exec(&mut gb);
        assert_eq!(gb.register.pc, 12);
    }

    #[test]
    fn it_should_not_jump_when_c_flag_is_set() {
        let mut gb = GameBoy::new();
        gb.register.f.set(Flags::C, true);
        let instruction = JumpRelative::nc(10);
        instruction.exec(&mut gb);
        assert_eq!(gb.register.pc, 2);
    }

    #[test]
    fn it_should_jump_when_c_flag_is_set() {
        let mut gb = GameBoy::new();
        gb.register.f.set(Flags::C, true);
        let instruction = JumpRelative::c(10);
        instruction.exec(&mut gb);
        assert_eq!(gb.register.pc, 12);
    }

    #[test]
    fn it_should_not_jump_when_c_flag_is_reset() {
        let mut gb = GameBoy::new();
        let instruction = JumpRelative::z(10);
        instruction.exec(&mut gb);
        assert_eq!(gb.register.pc, 2);
    }
}