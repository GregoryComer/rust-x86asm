use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovdqu_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 111, 231], OperandSize::Dword)
}

#[test]
fn vmovdqu_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(ECX, 46783514, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 111, 153, 26, 220, 201, 2], OperandSize::Dword)
}

#[test]
fn vmovdqu_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 111, 223], OperandSize::Qword)
}

#[test]
fn vmovdqu_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 1265324556, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 111, 20, 245, 12, 82, 107, 75], OperandSize::Qword)
}

#[test]
fn vmovdqu_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 111, 237], OperandSize::Dword)
}

#[test]
fn vmovdqu_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Direct(YMM6)), operand2: Some(IndirectDisplaced(ESI, 1523179420, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 111, 182, 156, 223, 201, 90], OperandSize::Dword)
}

#[test]
fn vmovdqu_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 111, 196], OperandSize::Qword)
}

#[test]
fn vmovdqu_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 1230619002, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 111, 28, 205, 122, 193, 89, 73], OperandSize::Qword)
}

#[test]
fn vmovdqu_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 111, 208], OperandSize::Dword)
}

#[test]
fn vmovdqu_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(IndirectScaledIndexed(EAX, ECX, Four, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 127, 28, 136], OperandSize::Dword)
}

#[test]
fn vmovdqu_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 111, 231], OperandSize::Qword)
}

#[test]
fn vmovdqu_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(IndirectScaledDisplaced(RDI, Two, 664848550, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 127, 52, 125, 166, 200, 160, 39], OperandSize::Qword)
}

#[test]
fn vmovdqu_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 111, 254], OperandSize::Dword)
}

#[test]
fn vmovdqu_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(IndirectScaledIndexed(EDI, ECX, Two, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 127, 20, 79], OperandSize::Dword)
}

#[test]
fn vmovdqu_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 111, 233], OperandSize::Qword)
}

#[test]
fn vmovdqu_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 127, 51], OperandSize::Qword)
}

