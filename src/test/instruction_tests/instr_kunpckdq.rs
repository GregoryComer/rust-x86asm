use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn kunpckdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KUNPCKDQ, operand1: Some(Direct(K2)), operand2: Some(Direct(K1)), operand3: Some(Direct(K7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 244, 75, 215], OperandSize::Dword)
}

#[test]
fn kunpckdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KUNPCKDQ, operand1: Some(Direct(K5)), operand2: Some(Direct(K7)), operand3: Some(Direct(K5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 196, 75, 237], OperandSize::Qword)
}

