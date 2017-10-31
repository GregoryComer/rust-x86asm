use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvtdq2pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTDQ2PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 230, 207], OperandSize::Dword)
}

#[test]
fn cvtdq2pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTDQ2PD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Two, 1591476699, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 230, 164, 64, 219, 1, 220, 94], OperandSize::Dword)
}

#[test]
fn cvtdq2pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTDQ2PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 230, 222], OperandSize::Qword)
}

#[test]
fn cvtdq2pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTDQ2PD, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 230, 42], OperandSize::Qword)
}

