use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn rcpps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::RCPPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 83, 219], OperandSize::Dword)
}

#[test]
fn rcpps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::RCPPS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Four, 147027933, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 83, 188, 150, 221, 119, 195, 8], OperandSize::Dword)
}

#[test]
fn rcpps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::RCPPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 83, 241], OperandSize::Qword)
}

#[test]
fn rcpps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::RCPPS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(RDI, 1776103490, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 83, 167, 66, 48, 221, 105], OperandSize::Qword)
}

