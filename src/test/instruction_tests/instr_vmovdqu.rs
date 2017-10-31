use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovdqu_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 111, 239], OperandSize::Dword)
}

#[test]
fn vmovdqu_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(ESI, 1687128323, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 111, 174, 3, 137, 143, 100], OperandSize::Dword)
}

#[test]
fn vmovdqu_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 111, 197], OperandSize::Qword)
}

#[test]
fn vmovdqu_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(RAX, 105624646, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 111, 144, 70, 180, 75, 6], OperandSize::Qword)
}

#[test]
fn vmovdqu_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 111, 233], OperandSize::Dword)
}

#[test]
fn vmovdqu_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Direct(YMM2)), operand2: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 111, 22], OperandSize::Dword)
}

#[test]
fn vmovdqu_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 111, 228], OperandSize::Qword)
}

#[test]
fn vmovdqu_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledDisplaced(RBX, Eight, 1118418673, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 111, 52, 221, 241, 182, 169, 66], OperandSize::Qword)
}

#[test]
fn vmovdqu_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 111, 198], OperandSize::Dword)
}

#[test]
fn vmovdqu_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(IndirectScaledDisplaced(EBX, Four, 431334906, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 127, 4, 157, 250, 165, 181, 25], OperandSize::Dword)
}

#[test]
fn vmovdqu_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 111, 213], OperandSize::Qword)
}

#[test]
fn vmovdqu_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(IndirectScaledIndexed(RBX, RAX, Four, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 127, 36, 131], OperandSize::Qword)
}

#[test]
fn vmovdqu_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 111, 215], OperandSize::Dword)
}

#[test]
fn vmovdqu_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Two, 921471039, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 127, 172, 91, 63, 136, 236, 54], OperandSize::Dword)
}

#[test]
fn vmovdqu_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 111, 230], OperandSize::Qword)
}

#[test]
fn vmovdqu_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 127, 38], OperandSize::Qword)
}

