use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn roundpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(55)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 9, 231, 55], OperandSize::Dword)
}

#[test]
fn roundpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDPD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(EBX, Four, 890294357, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(32)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 9, 44, 157, 85, 208, 16, 53, 32], OperandSize::Dword)
}

#[test]
fn roundpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(74)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 9, 254, 74], OperandSize::Qword)
}

#[test]
fn roundpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDPD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 1214724561, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(82)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 9, 44, 69, 209, 57, 103, 72, 82], OperandSize::Qword)
}

