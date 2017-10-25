use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmpxchg8b_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG8B, operand1: Some(IndirectDisplaced(SI, 167, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 199, 140, 167, 0], OperandSize::Word)
}

#[test]
fn cmpxchg8b_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG8B, operand1: Some(IndirectScaledDisplaced(EDI, Four, 2040282303, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 199, 12, 189, 191, 60, 156, 121], OperandSize::Dword)
}

#[test]
fn cmpxchg8b_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG8B, operand1: Some(IndirectScaledIndexed(RCX, RCX, Two, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 199, 12, 73], OperandSize::Qword)
}

