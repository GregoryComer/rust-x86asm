use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn unpckhpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKHPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 21, 244], OperandSize::Dword)
}

#[test]
fn unpckhpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKHPD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Two, 202318796, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 21, 140, 123, 204, 35, 15, 12], OperandSize::Dword)
}

#[test]
fn unpckhpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKHPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 21, 228], OperandSize::Qword)
}

#[test]
fn unpckhpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKHPD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Two, 1544088160, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 21, 164, 75, 96, 234, 8, 92], OperandSize::Qword)
}

