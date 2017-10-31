use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn kunpckdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KUNPCKDQ, operand1: Some(Direct(K1)), operand2: Some(Direct(K1)), operand3: Some(Direct(K6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 244, 75, 206], OperandSize::Dword)
}

#[test]
fn kunpckdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KUNPCKDQ, operand1: Some(Direct(K6)), operand2: Some(Direct(K2)), operand3: Some(Direct(K6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 236, 75, 246], OperandSize::Qword)
}

