use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovdqa_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 111, 233], OperandSize::Dword)
}

#[test]
fn vmovdqa_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(EAX, 928907728, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 111, 128, 208, 1, 94, 55], OperandSize::Dword)
}

#[test]
fn vmovdqa_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 111, 255], OperandSize::Qword)
}

#[test]
fn vmovdqa_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 1591789848, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 111, 12, 141, 24, 201, 224, 94], OperandSize::Qword)
}

#[test]
fn vmovdqa_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 111, 245], OperandSize::Dword)
}

#[test]
fn vmovdqa_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Four, 1013078433, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 111, 140, 178, 161, 89, 98, 60], OperandSize::Dword)
}

#[test]
fn vmovdqa_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 111, 233], OperandSize::Qword)
}

#[test]
fn vmovdqa_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledIndexed(RSI, RDI, Four, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 111, 52, 190], OperandSize::Qword)
}

#[test]
fn vmovdqa_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 111, 229], OperandSize::Dword)
}

#[test]
fn vmovdqa_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 127, 14], OperandSize::Dword)
}

#[test]
fn vmovdqa_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 111, 234], OperandSize::Qword)
}

#[test]
fn vmovdqa_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(IndirectDisplaced(RCX, 1154110240, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 127, 137, 32, 83, 202, 68], OperandSize::Qword)
}

#[test]
fn vmovdqa_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 111, 202], OperandSize::Dword)
}

#[test]
fn vmovdqa_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(IndirectDisplaced(ESI, 1043637885, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 127, 150, 125, 166, 52, 62], OperandSize::Dword)
}

#[test]
fn vmovdqa_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 111, 224], OperandSize::Qword)
}

#[test]
fn vmovdqa_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(IndirectDisplaced(RAX, 565395590, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 127, 152, 134, 64, 179, 33], OperandSize::Qword)
}

