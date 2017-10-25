use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvtdq2pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTDQ2PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 230, 215], OperandSize::Dword)
}

#[test]
fn cvtdq2pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTDQ2PD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(EDX, 3920694, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 230, 138, 54, 211, 59, 0], OperandSize::Dword)
}

#[test]
fn cvtdq2pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTDQ2PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 230, 216], OperandSize::Qword)
}

#[test]
fn cvtdq2pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTDQ2PD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 640422454, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 230, 4, 85, 54, 18, 44, 38], OperandSize::Qword)
}

