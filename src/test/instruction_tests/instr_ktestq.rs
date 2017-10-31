use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn ktestq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KTESTQ, operand1: Some(Direct(K4)), operand2: Some(Direct(K1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 248, 153, 225], OperandSize::Dword)
}

#[test]
fn ktestq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KTESTQ, operand1: Some(Direct(K2)), operand2: Some(Direct(K5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 248, 153, 213], OperandSize::Qword)
}

