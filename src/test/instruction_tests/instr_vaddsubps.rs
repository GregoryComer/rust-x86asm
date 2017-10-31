use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vaddsubps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 227, 208, 250], OperandSize::Dword)
}

#[test]
fn vaddsubps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 1334771755, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 235, 208, 20, 77, 43, 0, 143, 79], OperandSize::Dword)
}

#[test]
fn vaddsubps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 195, 208, 251], OperandSize::Qword)
}

#[test]
fn vaddsubps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Eight, 252107358, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 203, 208, 132, 208, 94, 218, 6, 15], OperandSize::Qword)
}

#[test]
fn vaddsubps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 208, 228], OperandSize::Dword)
}

#[test]
fn vaddsubps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(EDX, EDX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 208, 60, 82], OperandSize::Dword)
}

#[test]
fn vaddsubps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 231, 208, 217], OperandSize::Qword)
}

#[test]
fn vaddsubps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 199, 208, 15], OperandSize::Qword)
}

