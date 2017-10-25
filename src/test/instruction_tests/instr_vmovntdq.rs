use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovntdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQ, operand1: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 231, 3], OperandSize::Dword)
}

#[test]
fn vmovntdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQ, operand1: Some(IndirectScaledDisplaced(RSI, Four, 721038800, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 231, 28, 181, 208, 45, 250, 42], OperandSize::Qword)
}

#[test]
fn vmovntdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQ, operand1: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Two, 460737860, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 231, 164, 66, 68, 77, 118, 27], OperandSize::Dword)
}

#[test]
fn vmovntdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQ, operand1: Some(IndirectScaledIndexed(RAX, RDI, Two, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 231, 52, 120], OperandSize::Qword)
}

#[test]
fn vmovntdq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQ, operand1: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Four, 542696777, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 231, 148, 145, 73, 229, 88, 32], OperandSize::Dword)
}

#[test]
fn vmovntdq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQ, operand1: Some(IndirectScaledIndexed(RDX, RDX, Two, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 231, 44, 82], OperandSize::Qword)
}

#[test]
fn vmovntdq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQ, operand1: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Four, 1892850448, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 231, 172, 136, 16, 155, 210, 112], OperandSize::Dword)
}

#[test]
fn vmovntdq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQ, operand1: Some(IndirectScaledDisplaced(RDI, Eight, 1391761609, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 125, 231, 52, 253, 201, 152, 244, 82], OperandSize::Qword)
}

#[test]
fn vmovntdq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQ, operand1: Some(Indirect(ECX, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 125, 72, 231, 17], OperandSize::Dword)
}

#[test]
fn vmovntdq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQ, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Eight, 207552989, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 125, 72, 231, 172, 246, 221, 1, 95, 12], OperandSize::Qword)
}

