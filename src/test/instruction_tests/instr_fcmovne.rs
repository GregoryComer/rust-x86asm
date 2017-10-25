use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fcmovne_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FCMOVNE, operand1: Some(Direct(ST)), operand2: Some(Direct(ST7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 207], OperandSize::Word)
}

#[test]
fn fcmovne_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FCMOVNE, operand1: Some(Direct(ST)), operand2: Some(Direct(ST2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 202], OperandSize::Dword)
}

#[test]
fn fcmovne_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FCMOVNE, operand1: Some(Direct(ST)), operand2: Some(Direct(ST4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 204], OperandSize::Qword)
}

