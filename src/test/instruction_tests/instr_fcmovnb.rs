use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fcmovnb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FCMOVNB, operand1: Some(Direct(ST)), operand2: Some(Direct(ST4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 196], OperandSize::Word)
}

#[test]
fn fcmovnb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FCMOVNB, operand1: Some(Direct(ST)), operand2: Some(Direct(ST3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 195], OperandSize::Dword)
}

#[test]
fn fcmovnb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FCMOVNB, operand1: Some(Direct(ST)), operand2: Some(Direct(ST3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 195], OperandSize::Qword)
}

