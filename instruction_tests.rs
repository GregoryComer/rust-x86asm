use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

pub struct TestCase {
	pub addr_size: OperandSize,
	pub instruction: Instruction,
	pub expected: &'static [u8],
}

pub static INSTRUCTION_TESTS: [TestCase; 0] = [
];
