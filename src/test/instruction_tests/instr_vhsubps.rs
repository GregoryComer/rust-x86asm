use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vhsubps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 227, 125, 248], OperandSize::Dword)
}

#[test]
fn vhsubps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EDI, 497773853, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 203, 125, 175, 29, 109, 171, 29], OperandSize::Dword)
}

#[test]
fn vhsubps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 211, 125, 212], OperandSize::Qword)
}

#[test]
fn vhsubps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 1211689217, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 235, 125, 52, 117, 1, 233, 56, 72], OperandSize::Qword)
}

#[test]
fn vhsubps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 239, 125, 244], OperandSize::Dword)
}

#[test]
fn vhsubps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 125, 51], OperandSize::Dword)
}

#[test]
fn vhsubps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 199, 125, 194], OperandSize::Qword)
}

#[test]
fn vhsubps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(RSI, 795351081, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 125, 190, 41, 24, 104, 47], OperandSize::Qword)
}

