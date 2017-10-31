use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn prefetcht0_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHT0, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 24, 10], OperandSize::Word)
}

#[test]
fn prefetcht0_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHT0, operand1: Some(IndirectScaledDisplaced(ECX, Eight, 1518213497, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 24, 12, 205, 121, 25, 126, 90], OperandSize::Dword)
}

#[test]
fn prefetcht0_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHT0, operand1: Some(IndirectScaledIndexed(RBX, RSI, Two, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 24, 12, 115], OperandSize::Qword)
}

