use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn prefetcht0_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHT0, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 26517, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 24, 136, 149, 103], OperandSize::Word)
}

#[test]
fn prefetcht0_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHT0, operand1: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Eight, 1270889415, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 24, 140, 208, 199, 59, 192, 75], OperandSize::Dword)
}

#[test]
fn prefetcht0_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHT0, operand1: Some(IndirectScaledIndexed(RDI, RCX, Two, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 24, 12, 79], OperandSize::Qword)
}

