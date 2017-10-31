use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovdqa_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 111, 214], OperandSize::Dword)
}

#[test]
fn vmovdqa_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 111, 9], OperandSize::Dword)
}

#[test]
fn vmovdqa_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 111, 209], OperandSize::Qword)
}

#[test]
fn vmovdqa_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(RAX, 119249180, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 111, 176, 28, 153, 27, 7], OperandSize::Qword)
}

#[test]
fn vmovdqa_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 111, 207], OperandSize::Dword)
}

#[test]
fn vmovdqa_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Direct(YMM2)), operand2: Some(IndirectDisplaced(ESI, 557080309, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 111, 150, 245, 94, 52, 33], OperandSize::Dword)
}

#[test]
fn vmovdqa_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 111, 226], OperandSize::Qword)
}

#[test]
fn vmovdqa_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 827380718, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 111, 52, 205, 238, 211, 80, 49], OperandSize::Qword)
}

#[test]
fn vmovdqa_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 111, 223], OperandSize::Dword)
}

#[test]
fn vmovdqa_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Four, 855854635, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 127, 140, 155, 43, 78, 3, 51], OperandSize::Dword)
}

#[test]
fn vmovdqa_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 111, 244], OperandSize::Qword)
}

#[test]
fn vmovdqa_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(IndirectScaledIndexed(RDX, RSI, Four, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 127, 60, 178], OperandSize::Qword)
}

#[test]
fn vmovdqa_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 111, 254], OperandSize::Dword)
}

#[test]
fn vmovdqa_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(IndirectScaledDisplaced(ESI, Four, 1572981675, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 127, 4, 181, 171, 203, 193, 93], OperandSize::Dword)
}

#[test]
fn vmovdqa_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 111, 240], OperandSize::Qword)
}

#[test]
fn vmovdqa_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(IndirectDisplaced(RAX, 1173813753, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 127, 128, 249, 249, 246, 69], OperandSize::Qword)
}

