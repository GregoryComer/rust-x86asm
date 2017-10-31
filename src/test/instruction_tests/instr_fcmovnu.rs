use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fcmovnu_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FCMOVNU, operand1: Some(Direct(ST)), operand2: Some(Direct(ST6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 222], OperandSize::Word)
}

#[test]
fn fcmovnu_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FCMOVNU, operand1: Some(Direct(ST)), operand2: Some(Direct(ST7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 223], OperandSize::Dword)
}

#[test]
fn fcmovnu_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FCMOVNU, operand1: Some(Direct(ST)), operand2: Some(Direct(ST1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 217], OperandSize::Qword)
}

