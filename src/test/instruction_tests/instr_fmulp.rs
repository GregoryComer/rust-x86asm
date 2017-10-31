use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fmulp_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FMULP, operand1: Some(Direct(ST5)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 205], OperandSize::Word)
}

#[test]
fn fmulp_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FMULP, operand1: Some(Direct(ST5)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 205], OperandSize::Dword)
}

#[test]
fn fmulp_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FMULP, operand1: Some(Direct(ST7)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 207], OperandSize::Qword)
}

