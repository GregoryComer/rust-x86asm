use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovdqu_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 111, 211], OperandSize::Dword)
}

#[test]
fn vmovdqu_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(ECX, 572206223, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 111, 161, 143, 44, 27, 34], OperandSize::Dword)
}

#[test]
fn vmovdqu_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 111, 226], OperandSize::Qword)
}

#[test]
fn vmovdqu_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 1745072408, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 111, 20, 85, 24, 177, 3, 104], OperandSize::Qword)
}

#[test]
fn vmovdqu_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 111, 252], OperandSize::Dword)
}

#[test]
fn vmovdqu_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Direct(YMM0)), operand2: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 111, 1], OperandSize::Dword)
}

#[test]
fn vmovdqu_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 111, 216], OperandSize::Qword)
}

#[test]
fn vmovdqu_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexed(RCX, RCX, Four, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 111, 4, 137], OperandSize::Qword)
}

#[test]
fn vmovdqu_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 111, 195], OperandSize::Dword)
}

#[test]
fn vmovdqu_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Eight, 1824967362, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 127, 188, 207, 194, 202, 198, 108], OperandSize::Dword)
}

#[test]
fn vmovdqu_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 111, 255], OperandSize::Qword)
}

#[test]
fn vmovdqu_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(IndirectScaledIndexed(RDI, RDI, Eight, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 127, 44, 255], OperandSize::Qword)
}

#[test]
fn vmovdqu_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 111, 216], OperandSize::Dword)
}

#[test]
fn vmovdqu_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Eight, 930617767, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 127, 132, 217, 167, 25, 120, 55], OperandSize::Dword)
}

#[test]
fn vmovdqu_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 111, 200], OperandSize::Qword)
}

#[test]
fn vmovdqu_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(IndirectScaledIndexed(RCX, RSI, Four, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 127, 28, 177], OperandSize::Qword)
}

