use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fdivrp_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIVRP, operand1: Some(Direct(ST2)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 242], OperandSize::Word)
}

#[test]
fn fdivrp_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIVRP, operand1: Some(Direct(ST7)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 247], OperandSize::Dword)
}

#[test]
fn fdivrp_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIVRP, operand1: Some(Direct(ST7)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 247], OperandSize::Qword)
}

