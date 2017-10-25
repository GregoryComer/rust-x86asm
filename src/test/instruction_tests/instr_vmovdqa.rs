use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovdqa_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 111, 198], OperandSize::Dword)
}

#[test]
fn vmovdqa_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(EDI, 1194295454, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 111, 135, 158, 128, 47, 71], OperandSize::Dword)
}

#[test]
fn vmovdqa_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 111, 248], OperandSize::Qword)
}

#[test]
fn vmovdqa_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(RBX, 949689445, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 111, 155, 101, 28, 155, 56], OperandSize::Qword)
}

#[test]
fn vmovdqa_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 111, 234], OperandSize::Dword)
}

#[test]
fn vmovdqa_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Direct(YMM4)), operand2: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 111, 34], OperandSize::Dword)
}

#[test]
fn vmovdqa_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 111, 205], OperandSize::Qword)
}

#[test]
fn vmovdqa_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Direct(YMM5)), operand2: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 111, 43], OperandSize::Qword)
}

#[test]
fn vmovdqa_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 111, 230], OperandSize::Dword)
}

#[test]
fn vmovdqa_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(IndirectScaledIndexed(EAX, EBX, Four, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 127, 36, 152], OperandSize::Dword)
}

#[test]
fn vmovdqa_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 111, 235], OperandSize::Qword)
}

#[test]
fn vmovdqa_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(IndirectDisplaced(RAX, 647605675, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 127, 160, 171, 173, 153, 38], OperandSize::Qword)
}

#[test]
fn vmovdqa_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 111, 241], OperandSize::Dword)
}

#[test]
fn vmovdqa_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(IndirectDisplaced(EDX, 478981907, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 127, 170, 19, 175, 140, 28], OperandSize::Dword)
}

#[test]
fn vmovdqa_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 111, 223], OperandSize::Qword)
}

#[test]
fn vmovdqa_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(IndirectScaledIndexed(RCX, RSI, Four, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 127, 4, 177], OperandSize::Qword)
}

