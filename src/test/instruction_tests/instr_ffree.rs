use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn ffree_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FFREE, operand1: Some(Direct(ST4)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 196], OperandSize::Word)
}

#[test]
fn ffree_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FFREE, operand1: Some(Direct(ST2)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 194], OperandSize::Dword)
}

#[test]
fn ffree_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FFREE, operand1: Some(Direct(ST7)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 199], OperandSize::Qword)
}

