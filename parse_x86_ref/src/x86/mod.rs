pub mod instruction_def;
pub mod instruction;
pub mod mnemonic;

mod instruction_buffer;
#[cfg(test)] mod test;

pub use self::mnemonic::Mnemonic;
pub use self::instruction::{ Instruction, Operand, Reg, RegScale, InstructionFlags, OperandSize, SegmentReg } ;

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

#[derive(Copy, Clone, Debug)]
pub enum BroadcastMode {}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Mode {
    Real,
    Protected,
    Long,
    SystemManagement // TODO
}

impl Mode {
    fn pointer_size(&self) -> OperandSize {
        match *self {
            Mode::Real => OperandSize::Word,
            Mode::Protected => OperandSize::Dword,
            Mode::Long => OperandSize::Qword,
            Mode::SystemManagement => OperandSize::Word // TODO
        }
    }
}

pub fn load() {
    instruction_def::load_instructions();
}
