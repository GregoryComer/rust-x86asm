use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fcmovnbe_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FCMOVNBE, operand1: Some(Direct(ST)), operand2: Some(Direct(ST6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 214], OperandSize::Word)
}

#[test]
fn fcmovnbe_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FCMOVNBE, operand1: Some(Direct(ST)), operand2: Some(Direct(ST1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 209], OperandSize::Dword)
}

#[test]
fn fcmovnbe_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FCMOVNBE, operand1: Some(Direct(ST)), operand2: Some(Direct(ST6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 214], OperandSize::Qword)
}

