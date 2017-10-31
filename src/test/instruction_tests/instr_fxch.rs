use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fxch_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FXCH, operand1: Some(Direct(ST7)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 207], OperandSize::Word)
}

#[test]
fn fxch_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FXCH, operand1: Some(Direct(ST6)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 206], OperandSize::Dword)
}

#[test]
fn fxch_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FXCH, operand1: Some(Direct(ST6)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 206], OperandSize::Qword)
}

