use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn kunpckwd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KUNPCKWD, operand1: Some(Direct(K6)), operand2: Some(Direct(K1)), operand3: Some(Direct(K4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 244, 75, 244], OperandSize::Dword)
}

#[test]
fn kunpckwd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KUNPCKWD, operand1: Some(Direct(K2)), operand2: Some(Direct(K6)), operand3: Some(Direct(K7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 204, 75, 215], OperandSize::Qword)
}

