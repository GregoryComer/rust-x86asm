use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fsubp_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBP, operand1: Some(Direct(ST7)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 239], OperandSize::Word)
}

#[test]
fn fsubp_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBP, operand1: Some(Direct(ST3)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 235], OperandSize::Dword)
}

#[test]
fn fsubp_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBP, operand1: Some(Direct(ST6)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 238], OperandSize::Qword)
}

