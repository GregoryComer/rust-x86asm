use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fucomp_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FUCOMP, operand1: Some(Direct(ST6)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 238], OperandSize::Word)
}

#[test]
fn fucomp_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FUCOMP, operand1: Some(Direct(ST6)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 238], OperandSize::Dword)
}

#[test]
fn fucomp_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FUCOMP, operand1: Some(Direct(ST3)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 235], OperandSize::Qword)
}

