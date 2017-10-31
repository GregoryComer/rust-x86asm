use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vptest_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTEST, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 23, 227], OperandSize::Dword)
}

#[test]
fn vptest_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTEST, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(EDX, EDX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 23, 44, 82], OperandSize::Dword)
}

#[test]
fn vptest_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTEST, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 23, 243], OperandSize::Qword)
}

#[test]
fn vptest_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTEST, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(RSI, Four, 1787220630, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 23, 20, 181, 150, 210, 134, 106], OperandSize::Qword)
}

#[test]
fn vptest_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTEST, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 23, 213], OperandSize::Dword)
}

#[test]
fn vptest_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTEST, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Eight, 1107878650, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 23, 164, 250, 250, 226, 8, 66], OperandSize::Dword)
}

#[test]
fn vptest_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTEST, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 23, 246], OperandSize::Qword)
}

#[test]
fn vptest_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTEST, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 2027225443, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 23, 52, 205, 99, 1, 213, 120], OperandSize::Qword)
}

