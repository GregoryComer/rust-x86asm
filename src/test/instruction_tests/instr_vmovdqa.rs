use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovdqa_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 111, 217], OperandSize::Dword)
}

#[test]
fn vmovdqa_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 111, 32], OperandSize::Dword)
}

#[test]
fn vmovdqa_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 111, 200], OperandSize::Qword)
}

#[test]
fn vmovdqa_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(RBX, RSI, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 111, 4, 115], OperandSize::Qword)
}

#[test]
fn vmovdqa_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 111, 251], OperandSize::Dword)
}

#[test]
fn vmovdqa_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexed(EBX, EBX, Two, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 111, 36, 91], OperandSize::Dword)
}

#[test]
fn vmovdqa_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 111, 213], OperandSize::Qword)
}

#[test]
fn vmovdqa_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Direct(YMM6)), operand2: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 111, 55], OperandSize::Qword)
}

#[test]
fn vmovdqa_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 111, 254], OperandSize::Dword)
}

#[test]
fn vmovdqa_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Two, 103095948, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 127, 148, 112, 140, 30, 37, 6], OperandSize::Dword)
}

#[test]
fn vmovdqa_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 111, 195], OperandSize::Qword)
}

#[test]
fn vmovdqa_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 127, 6], OperandSize::Qword)
}

#[test]
fn vmovdqa_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 111, 202], OperandSize::Dword)
}

#[test]
fn vmovdqa_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(IndirectScaledIndexed(EBX, ECX, Eight, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 127, 12, 203], OperandSize::Dword)
}

#[test]
fn vmovdqa_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 111, 253], OperandSize::Qword)
}

#[test]
fn vmovdqa_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(IndirectScaledDisplaced(RCX, Eight, 373178125, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 127, 36, 205, 13, 63, 62, 22], OperandSize::Qword)
}

