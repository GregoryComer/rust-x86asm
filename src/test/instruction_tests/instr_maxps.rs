use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn maxps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 95, 251], OperandSize::Dword)
}

#[test]
fn maxps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXPS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 1676768325, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 95, 12, 77, 69, 116, 241, 99], OperandSize::Dword)
}

#[test]
fn maxps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 95, 239], OperandSize::Qword)
}

#[test]
fn maxps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXPS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(RCX, 1655938997, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 95, 185, 181, 159, 179, 98], OperandSize::Qword)
}

