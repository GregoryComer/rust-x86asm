use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovntdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQ, operand1: Some(IndirectScaledDisplaced(ECX, Two, 853055725, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 231, 36, 77, 237, 152, 216, 50], OperandSize::Dword)
}

#[test]
fn vmovntdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQ, operand1: Some(IndirectScaledIndexed(RSI, RAX, Four, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 231, 36, 134], OperandSize::Qword)
}

#[test]
fn vmovntdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQ, operand1: Some(IndirectDisplaced(ESI, 1199967223, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 231, 150, 247, 11, 134, 71], OperandSize::Dword)
}

#[test]
fn vmovntdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQ, operand1: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 231, 9], OperandSize::Qword)
}

#[test]
fn vmovntdq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQ, operand1: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 231, 58], OperandSize::Dword)
}

#[test]
fn vmovntdq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQ, operand1: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 121, 231, 2], OperandSize::Qword)
}

#[test]
fn vmovntdq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQ, operand1: Some(IndirectScaledIndexed(EDI, EDX, Eight, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 231, 44, 215], OperandSize::Dword)
}

#[test]
fn vmovntdq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQ, operand1: Some(IndirectScaledDisplaced(RAX, Four, 62997739, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 231, 36, 133, 235, 68, 193, 3], OperandSize::Qword)
}

#[test]
fn vmovntdq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQ, operand1: Some(IndirectDisplaced(EAX, 1467895939, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 125, 72, 231, 128, 131, 80, 126, 87], OperandSize::Dword)
}

#[test]
fn vmovntdq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQ, operand1: Some(IndirectScaledIndexed(RCX, RCX, Two, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 113, 125, 72, 231, 52, 73], OperandSize::Qword)
}

