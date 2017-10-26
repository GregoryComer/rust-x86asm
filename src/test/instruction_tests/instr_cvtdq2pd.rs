use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvtdq2pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTDQ2PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 230, 241], OperandSize::Dword)
}

#[test]
fn cvtdq2pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTDQ2PD, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 230, 26], OperandSize::Dword)
}

#[test]
fn cvtdq2pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTDQ2PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 230, 229], OperandSize::Qword)
}

#[test]
fn cvtdq2pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTDQ2PD, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 230, 48], OperandSize::Qword)
}

