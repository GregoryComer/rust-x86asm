extern crate byteorder;
#[macro_use] extern crate lazy_static;
extern crate arrayvec;

mod decoding;
mod encoding;
mod instruction;
mod instruction_buffer;
mod instruction_def;
mod instruction_defs;
mod mnemonic;
mod operand;
#[cfg(test)] mod test;

pub use self::decoding::{InstructionDecodingError, InstructionReader};
pub use self::encoding::{InstructionEncodingError, InstructionWriter};
pub use self::instruction::{ Instruction, Reg, RegScale, SegmentReg, MergeMode, MaskReg, BroadcastMode, RoundingMode };
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

    fn from_size(size: OperandSize) -> Option<Mode> {
        match size {
            OperandSize::Word => Some(Mode::Real),
            OperandSize::Dword => Some(Mode::Protected),
            OperandSize::Qword => Some(Mode::Long),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum RegType {
    General,
    Mmx,
    Avx,
    Fpu,
    Bound,
    Mask,
    Segment
}
