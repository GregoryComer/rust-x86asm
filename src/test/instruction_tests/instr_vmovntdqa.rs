use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovntdqa_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQA, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Four, 176566229, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 42, 148, 176, 213, 47, 134, 10], OperandSize::Dword)
}

#[test]
fn vmovntdqa_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQA, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Four, 383181311, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 42, 148, 183, 255, 225, 214, 22], OperandSize::Qword)
}

#[test]
fn vmovntdqa_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQA, operand1: Some(Direct(YMM7)), operand2: Some(IndirectDisplaced(ESI, 933380979, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 42, 190, 115, 67, 162, 55], OperandSize::Dword)
}

#[test]
fn vmovntdqa_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQA, operand1: Some(Direct(YMM1)), operand2: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 42, 8], OperandSize::Qword)
}

#[test]
fn vmovntdqa_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQA, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Two, 2015190345, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 42, 156, 78, 73, 93, 29, 120], OperandSize::Dword)
}

#[test]
fn vmovntdqa_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQA, operand1: Some(Direct(XMM16)), operand2: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 125, 8, 42, 3], OperandSize::Qword)
}

#[test]
fn vmovntdqa_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQA, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 532127134, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 42, 4, 77, 158, 157, 183, 31], OperandSize::Dword)
}

#[test]
fn vmovntdqa_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQA, operand1: Some(Direct(YMM20)), operand2: Some(IndirectDisplaced(RSI, 940941355, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 125, 40, 42, 166, 43, 160, 21, 56], OperandSize::Qword)
}

#[test]
fn vmovntdqa_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQA, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectDisplaced(EDX, 956820301, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 125, 72, 42, 178, 77, 235, 7, 57], OperandSize::Dword)
}

#[test]
fn vmovntdqa_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQA, operand1: Some(Direct(ZMM10)), operand2: Some(IndirectScaledIndexed(RSI, RBX, Eight, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 125, 72, 42, 20, 222], OperandSize::Qword)
}

