use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn maxps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 95, 252], OperandSize::Dword)
}

#[test]
fn maxps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXPS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 346118717, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 95, 60, 69, 61, 90, 161, 20], OperandSize::Dword)
}

#[test]
fn maxps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 95, 211], OperandSize::Qword)
}

#[test]
fn maxps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXPS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 1941983422, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 95, 44, 213, 190, 80, 192, 115], OperandSize::Qword)
}

