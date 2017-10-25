use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vptest_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTEST, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 23, 253], OperandSize::Dword)
}

#[test]
fn vptest_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTEST, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 847033930, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 23, 20, 133, 74, 182, 124, 50], OperandSize::Dword)
}

#[test]
fn vptest_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTEST, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 23, 204], OperandSize::Qword)
}

#[test]
fn vptest_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTEST, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Eight, 1696516720, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 23, 140, 202, 112, 202, 30, 101], OperandSize::Qword)
}

#[test]
fn vptest_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTEST, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 23, 217], OperandSize::Dword)
}

#[test]
fn vptest_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTEST, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledDisplaced(EDX, Two, 86614745, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 23, 12, 85, 217, 162, 41, 5], OperandSize::Dword)
}

#[test]
fn vptest_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTEST, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 23, 209], OperandSize::Qword)
}

#[test]
fn vptest_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTEST, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexed(RDI, RDI, Four, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 23, 44, 191], OperandSize::Qword)
}

