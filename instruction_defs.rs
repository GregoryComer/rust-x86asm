use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::instruction_def::OperandType::*;
use ::instruction_def::OperandSizePrefixBehavior::*;
use ::Reg::*;
use ::RegScale::*;

pub struct TestCase {
	addr_size: OperandSize,
	instruction: Instruction,
	expected: &'static [u8],
}
pub static INSTR_DEFS: [InstructionDefinition; 0] = [
];
