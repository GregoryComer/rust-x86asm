use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fnsave_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSAVE, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 109, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 115, 109], OperandSize::Word)
}

#[test]
fn fnsave_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSAVE, operand1: Some(Indirect(EDI, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 55], OperandSize::Dword)
}

#[test]
fn fnsave_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSAVE, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Two, 1052664475, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 180, 64, 155, 98, 190, 62], OperandSize::Qword)
}

