extern crate byteorder;
#[macro_use] extern crate lazy_static;

mod encoding;
mod instruction;
mod instruction_buffer;
mod instruction_def;
mod instruction_defs;
mod mnemonic;
mod operand;
#[cfg(test)] mod test;

pub use self::encoding::{InstructionEncodingError, InstructionWriter};
pub use self::instruction::{ Instruction, Reg, RegScale, InstructionFlags, SegmentReg, MergeMode, MaskReg, BroadcastMode, RoundingMode };
pub use self::operand::{Operand, OperandSize};
pub use self::mnemonic::Mnemonic;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Mode {
    Real,
    Protected,
    Long,
}

impl Mode {
    fn pointer_size(&self) -> OperandSize {
        match *self {
            Mode::Real => OperandSize::Word,
            Mode::Protected => OperandSize::Dword,
            Mode::Long => OperandSize::Qword,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProcessorLevel {
    i8086 = 0,
    i80186 = 1,
    i80286 = 2,
    i80386 = 3,
    i80486 = 4,
    Pentium1 = 5,
    Pentium1Mmx = 6,
    PentiumPro = 7,
    Pentium2 = 8,
    Pentium3 = 9,
    Pentium4 = 10,
    Core1 = 11,
    Core2 = 12,
    Corei7 = 13
}
