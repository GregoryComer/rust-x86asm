use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fucom_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FUCOM, operand1: Some(Direct(ST5)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 229], OperandSize::Word)
}

#[test]
fn fucom_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FUCOM, operand1: Some(Direct(ST6)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 230], OperandSize::Dword)
}

#[test]
fn fucom_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FUCOM, operand1: Some(Direct(ST1)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 225], OperandSize::Qword)
}

