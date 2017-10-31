use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn kortestd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KORTESTD, operand1: Some(Direct(K2)), operand2: Some(Direct(K4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 249, 152, 212], OperandSize::Dword)
}

#[test]
fn kortestd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KORTESTD, operand1: Some(Direct(K1)), operand2: Some(Direct(K6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 249, 152, 206], OperandSize::Qword)
}

