#[macro_use]
mod macros;
pub mod register;
pub mod instructions;
mod instruction;

pub use self::instruction::Instruction;