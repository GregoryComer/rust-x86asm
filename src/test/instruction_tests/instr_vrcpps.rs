use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrcpps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCPPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 83, 225], OperandSize::Dword)
}

#[test]
fn vrcpps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCPPS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(EDX, 1397059296, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 83, 170, 224, 110, 69, 83], OperandSize::Dword)
}

#[test]
fn vrcpps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCPPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 83, 208], OperandSize::Qword)
}

#[test]
fn vrcpps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCPPS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(RCX, Two, 957272222, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 83, 60, 77, 158, 208, 14, 57], OperandSize::Qword)
}

#[test]
fn vrcpps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCPPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 83, 196], OperandSize::Dword)
}

#[test]
fn vrcpps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCPPS, operand1: Some(Direct(YMM7)), operand2: Some(IndirectDisplaced(ESI, 297726130, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 83, 190, 178, 240, 190, 17], OperandSize::Dword)
}

#[test]
fn vrcpps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCPPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 83, 227], OperandSize::Qword)
}

#[test]
fn vrcpps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCPPS, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Two, 771946252, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 83, 140, 114, 12, 247, 2, 46], OperandSize::Qword)
}

