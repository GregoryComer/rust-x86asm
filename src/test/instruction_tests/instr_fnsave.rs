use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fnsave_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSAVE, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 12680, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 179, 136, 49], OperandSize::Word)
}

#[test]
fn fnsave_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSAVE, operand1: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Four, 93406966, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 180, 131, 246, 70, 145, 5], OperandSize::Dword)
}

#[test]
fn fnsave_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSAVE, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Two, 1264961757, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 180, 75, 221, 200, 101, 75], OperandSize::Qword)
}

