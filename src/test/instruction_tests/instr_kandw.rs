use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn kandw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KANDW, operand1: Some(Direct(K7)), operand2: Some(Direct(K4)), operand3: Some(Direct(K2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 220, 65, 250], OperandSize::Dword)
}

#[test]
fn kandw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KANDW, operand1: Some(Direct(K4)), operand2: Some(Direct(K1)), operand3: Some(Direct(K1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 244, 65, 225], OperandSize::Qword)
}

