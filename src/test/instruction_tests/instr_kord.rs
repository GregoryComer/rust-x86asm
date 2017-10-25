use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn kord_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KORD, operand1: Some(Direct(K3)), operand2: Some(Direct(K2)), operand3: Some(Direct(K6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 237, 69, 222], OperandSize::Dword)
}

#[test]
fn kord_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KORD, operand1: Some(Direct(K5)), operand2: Some(Direct(K6)), operand3: Some(Direct(K6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 205, 69, 238], OperandSize::Qword)
}

