use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn korw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KORW, operand1: Some(Direct(K7)), operand2: Some(Direct(K7)), operand3: Some(Direct(K2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 196, 69, 250], OperandSize::Dword)
}

#[test]
fn korw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KORW, operand1: Some(Direct(K2)), operand2: Some(Direct(K2)), operand3: Some(Direct(K5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 236, 69, 213], OperandSize::Qword)
}

