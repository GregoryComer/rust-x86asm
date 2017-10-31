use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovntdqa_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQA, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Four, 1770220439, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 42, 140, 144, 151, 107, 131, 105], OperandSize::Dword)
}

#[test]
fn vmovntdqa_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQA, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(RBX, 44940636, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 42, 187, 92, 189, 173, 2], OperandSize::Qword)
}

#[test]
fn vmovntdqa_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQA, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledIndexed(EBX, ESI, Four, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 42, 52, 179], OperandSize::Dword)
}

#[test]
fn vmovntdqa_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQA, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 667072892, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 42, 12, 133, 124, 185, 194, 39], OperandSize::Qword)
}

#[test]
fn vmovntdqa_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQA, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(EDI, 416088942, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 42, 183, 110, 3, 205, 24], OperandSize::Dword)
}

#[test]
fn vmovntdqa_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQA, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 205319726, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 42, 60, 141, 46, 238, 60, 12], OperandSize::Qword)
}

#[test]
fn vmovntdqa_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQA, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Two, 950541988, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 42, 140, 94, 164, 30, 168, 56], OperandSize::Dword)
}

#[test]
fn vmovntdqa_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQA, operand1: Some(Direct(YMM14)), operand2: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 98, 125, 42, 48], OperandSize::Qword)
}

#[test]
fn vmovntdqa_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQA, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Eight, 1136819810, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 125, 72, 42, 172, 223, 98, 126, 194, 67], OperandSize::Dword)
}

#[test]
fn vmovntdqa_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQA, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectDisplaced(RSI, 1870587378, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 125, 72, 42, 174, 242, 229, 126, 111], OperandSize::Qword)
}

