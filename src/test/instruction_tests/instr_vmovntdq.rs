use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovntdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQ, operand1: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 231, 46], OperandSize::Dword)
}

#[test]
fn vmovntdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQ, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Four, 783647599, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 231, 188, 135, 111, 131, 181, 46], OperandSize::Qword)
}

#[test]
fn vmovntdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQ, operand1: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Two, 258472182, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 231, 180, 75, 246, 248, 103, 15], OperandSize::Dword)
}

#[test]
fn vmovntdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQ, operand1: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 231, 8], OperandSize::Qword)
}

#[test]
fn vmovntdq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQ, operand1: Some(IndirectDisplaced(EDI, 1107366814, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 231, 191, 158, 19, 1, 66], OperandSize::Dword)
}

#[test]
fn vmovntdq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQ, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Four, 2004028035, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 121, 231, 180, 191, 131, 10, 115, 119], OperandSize::Qword)
}

#[test]
fn vmovntdq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQ, operand1: Some(IndirectScaledIndexed(ESI, EDI, Two, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 231, 28, 126], OperandSize::Dword)
}

#[test]
fn vmovntdq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQ, operand1: Some(IndirectDisplaced(RDI, 202434177, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 125, 231, 175, 129, 230, 16, 12], OperandSize::Qword)
}

#[test]
fn vmovntdq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQ, operand1: Some(Indirect(EDX, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 125, 72, 231, 42], OperandSize::Dword)
}

#[test]
fn vmovntdq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQ, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Eight, 1874225035, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 125, 72, 231, 140, 203, 139, 103, 182, 111], OperandSize::Qword)
}

