use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vphsubd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 6, 210], OperandSize::Dword)
}

#[test]
fn vphsubd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(ESI, ECX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 6, 28, 142], OperandSize::Dword)
}

#[test]
fn vphsubd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 6, 221], OperandSize::Qword)
}

#[test]
fn vphsubd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 1030113646, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 6, 60, 117, 110, 73, 102, 61], OperandSize::Qword)
}

#[test]
fn vphsubd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 6, 231], OperandSize::Dword)
}

#[test]
fn vphsubd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 263367729, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 6, 12, 69, 49, 172, 178, 15], OperandSize::Dword)
}

#[test]
fn vphsubd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 6, 237], OperandSize::Qword)
}

#[test]
fn vphsubd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(RBX, RDX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 6, 28, 83], OperandSize::Qword)
}

