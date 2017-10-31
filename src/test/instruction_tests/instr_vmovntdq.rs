use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovntdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQ, operand1: Some(IndirectScaledDisplaced(EDI, Eight, 1062671387, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 231, 4, 253, 27, 20, 87, 63], OperandSize::Dword)
}

#[test]
fn vmovntdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQ, operand1: Some(IndirectScaledIndexed(RAX, RAX, Four, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 231, 36, 128], OperandSize::Qword)
}

#[test]
fn vmovntdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQ, operand1: Some(IndirectDisplaced(EAX, 1083787311, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 231, 128, 47, 72, 153, 64], OperandSize::Dword)
}

#[test]
fn vmovntdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQ, operand1: Some(IndirectScaledIndexed(RBX, RCX, Eight, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 231, 36, 203], OperandSize::Qword)
}

#[test]
fn vmovntdq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQ, operand1: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 231, 49], OperandSize::Dword)
}

#[test]
fn vmovntdq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQ, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Two, 1992018460, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 121, 231, 180, 123, 28, 202, 187, 118], OperandSize::Qword)
}

#[test]
fn vmovntdq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQ, operand1: Some(IndirectDisplaced(EDX, 787292197, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 231, 162, 37, 32, 237, 46], OperandSize::Dword)
}

#[test]
fn vmovntdq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQ, operand1: Some(IndirectScaledIndexed(RAX, RDX, Eight, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 231, 12, 208], OperandSize::Qword)
}

#[test]
fn vmovntdq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQ, operand1: Some(IndirectDisplaced(EAX, 1138235764, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 125, 72, 231, 160, 116, 25, 216, 67], OperandSize::Dword)
}

#[test]
fn vmovntdq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQ, operand1: Some(Indirect(RBX, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 125, 72, 231, 27], OperandSize::Qword)
}

