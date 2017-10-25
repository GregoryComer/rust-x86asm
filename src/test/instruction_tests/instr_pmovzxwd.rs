use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmovzxwd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXWD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 51, 209], OperandSize::Dword)
}

#[test]
fn pmovzxwd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXWD, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 51, 63], OperandSize::Dword)
}

#[test]
fn pmovzxwd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXWD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 51, 252], OperandSize::Qword)
}

#[test]
fn pmovzxwd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXWD, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 51, 0], OperandSize::Qword)
}

