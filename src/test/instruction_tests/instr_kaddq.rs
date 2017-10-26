use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn kaddq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KADDQ, operand1: Some(Direct(K1)), operand2: Some(Direct(K4)), operand3: Some(Direct(K2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 220, 74, 202], OperandSize::Dword)
}

#[test]
fn kaddq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KADDQ, operand1: Some(Direct(K5)), operand2: Some(Direct(K4)), operand3: Some(Direct(K4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 220, 74, 236], OperandSize::Qword)
}

