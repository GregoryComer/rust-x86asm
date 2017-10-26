use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn unpckhpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKHPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 21, 252], OperandSize::Dword)
}

#[test]
fn unpckhpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKHPD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(ECX, 1298658509, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 21, 161, 205, 244, 103, 77], OperandSize::Dword)
}

#[test]
fn unpckhpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKHPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 21, 246], OperandSize::Qword)
}

#[test]
fn unpckhpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKHPD, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 21, 54], OperandSize::Qword)
}

