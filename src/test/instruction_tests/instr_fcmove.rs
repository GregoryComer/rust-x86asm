use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fcmove_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FCMOVE, operand1: Some(Direct(ST)), operand2: Some(Direct(ST5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 205], OperandSize::Word)
}

#[test]
fn fcmove_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FCMOVE, operand1: Some(Direct(ST)), operand2: Some(Direct(ST6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 206], OperandSize::Dword)
}

#[test]
fn fcmove_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FCMOVE, operand1: Some(Direct(ST)), operand2: Some(Direct(ST2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 202], OperandSize::Qword)
}

