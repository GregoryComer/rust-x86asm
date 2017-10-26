use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fcmovu_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FCMOVU, operand1: Some(Direct(ST)), operand2: Some(Direct(ST1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 217], OperandSize::Word)
}

#[test]
fn fcmovu_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FCMOVU, operand1: Some(Direct(ST)), operand2: Some(Direct(ST2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 218], OperandSize::Dword)
}

#[test]
fn fcmovu_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FCMOVU, operand1: Some(Direct(ST)), operand2: Some(Direct(ST5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 221], OperandSize::Qword)
}

