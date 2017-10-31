use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn kandq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KANDQ, operand1: Some(Direct(K3)), operand2: Some(Direct(K5)), operand3: Some(Direct(K1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 212, 65, 217], OperandSize::Dword)
}

#[test]
fn kandq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KANDQ, operand1: Some(Direct(K2)), operand2: Some(Direct(K6)), operand3: Some(Direct(K3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 204, 65, 211], OperandSize::Qword)
}

