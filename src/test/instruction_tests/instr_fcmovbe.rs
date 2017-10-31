use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fcmovbe_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FCMOVBE, operand1: Some(Direct(ST)), operand2: Some(Direct(ST3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 211], OperandSize::Word)
}

#[test]
fn fcmovbe_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FCMOVBE, operand1: Some(Direct(ST)), operand2: Some(Direct(ST1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 209], OperandSize::Dword)
}

#[test]
fn fcmovbe_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FCMOVBE, operand1: Some(Direct(ST)), operand2: Some(Direct(ST5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 213], OperandSize::Qword)
}

