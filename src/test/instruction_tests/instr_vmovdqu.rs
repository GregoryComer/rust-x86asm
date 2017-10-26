use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovdqu_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 111, 206], OperandSize::Dword)
}

#[test]
fn vmovdqu_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 111, 23], OperandSize::Dword)
}

#[test]
fn vmovdqu_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 111, 228], OperandSize::Qword)
}

#[test]
fn vmovdqu_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(RDX, 21174486, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 111, 138, 214, 24, 67, 1], OperandSize::Qword)
}

#[test]
fn vmovdqu_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 111, 203], OperandSize::Dword)
}

#[test]
fn vmovdqu_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Direct(YMM3)), operand2: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 111, 26], OperandSize::Dword)
}

#[test]
fn vmovdqu_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 111, 240], OperandSize::Qword)
}

#[test]
fn vmovdqu_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexed(RSI, RSI, Two, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 111, 12, 118], OperandSize::Qword)
}

#[test]
fn vmovdqu_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 111, 225], OperandSize::Dword)
}

#[test]
fn vmovdqu_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(IndirectScaledIndexed(EDI, EDI, Two, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 127, 52, 127], OperandSize::Dword)
}

#[test]
fn vmovdqu_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 111, 244], OperandSize::Qword)
}

#[test]
fn vmovdqu_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(IndirectScaledIndexed(RCX, RCX, Eight, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 127, 36, 201], OperandSize::Qword)
}

#[test]
fn vmovdqu_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 111, 250], OperandSize::Dword)
}

#[test]
fn vmovdqu_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Two, 1385254005, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 127, 132, 95, 117, 76, 145, 82], OperandSize::Dword)
}

#[test]
fn vmovdqu_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 111, 205], OperandSize::Qword)
}

#[test]
fn vmovdqu_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Eight, 897827067, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 127, 188, 194, 251, 192, 131, 53], OperandSize::Qword)
}

