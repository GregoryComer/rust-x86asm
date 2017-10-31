use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fucomip_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FUCOMIP, operand1: Some(Direct(ST)), operand2: Some(Direct(ST2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 234], OperandSize::Word)
}

#[test]
fn fucomip_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FUCOMIP, operand1: Some(Direct(ST)), operand2: Some(Direct(ST3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 235], OperandSize::Dword)
}

#[test]
fn fucomip_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FUCOMIP, operand1: Some(Direct(ST)), operand2: Some(Direct(ST7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 239], OperandSize::Qword)
}

