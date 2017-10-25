use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vphsubsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBSW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 7, 214], OperandSize::Dword)
}

#[test]
fn vphsubsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBSW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 1867148738, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 7, 44, 149, 194, 109, 74, 111], OperandSize::Dword)
}

#[test]
fn vphsubsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBSW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 7, 244], OperandSize::Qword)
}

#[test]
fn vphsubsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(RAX, 205891860, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 7, 128, 20, 169, 69, 12], OperandSize::Qword)
}

#[test]
fn vphsubsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBSW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 7, 248], OperandSize::Dword)
}

#[test]
fn vphsubsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBSW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Two, 902229356, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 7, 132, 67, 108, 237, 198, 53], OperandSize::Dword)
}

#[test]
fn vphsubsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBSW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 7, 213], OperandSize::Qword)
}

#[test]
fn vphsubsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBSW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(RCX, RCX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 7, 12, 73], OperandSize::Qword)
}

