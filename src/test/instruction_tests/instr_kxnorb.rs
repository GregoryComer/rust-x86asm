use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn kxnorb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KXNORB, operand1: Some(Direct(K1)), operand2: Some(Direct(K4)), operand3: Some(Direct(K4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 70, 204], OperandSize::Dword)
}

#[test]
fn kxnorb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KXNORB, operand1: Some(Direct(K1)), operand2: Some(Direct(K2)), operand3: Some(Direct(K5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 70, 205], OperandSize::Qword)
}

