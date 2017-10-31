use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vaddsubps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 208, 199], OperandSize::Dword)
}

#[test]
fn vaddsubps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(EDI, ESI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 243, 208, 52, 119], OperandSize::Dword)
}

#[test]
fn vaddsubps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 211, 208, 221], OperandSize::Qword)
}

#[test]
fn vaddsubps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Two, 1840238623, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 203, 208, 156, 83, 31, 208, 175, 109], OperandSize::Qword)
}

#[test]
fn vaddsubps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 208, 200], OperandSize::Dword)
}

#[test]
fn vaddsubps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(EDI, 292884063, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 207, 208, 183, 95, 14, 117, 17], OperandSize::Dword)
}

#[test]
fn vaddsubps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 247, 208, 201], OperandSize::Qword)
}

#[test]
fn vaddsubps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Two, 97477349, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 208, 172, 118, 229, 98, 207, 5], OperandSize::Qword)
}

