use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovdqu16_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 255, 137, 111, 222], OperandSize::Dword)
}

#[test]
fn vmovdqu16_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Eight, 758108643, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 255, 139, 111, 156, 203, 227, 209, 47, 45], OperandSize::Dword)
}

#[test]
fn vmovdqu16_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 49, 255, 137, 111, 249], OperandSize::Qword)
}

#[test]
fn vmovdqu16_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(XMM10)), operand2: Some(IndirectDisplaced(RCX, 120541877, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 255, 140, 111, 145, 181, 82, 47, 7], OperandSize::Qword)
}

#[test]
fn vmovdqu16_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 255, 170, 111, 243], OperandSize::Dword)
}

#[test]
fn vmovdqu16_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 64170312, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 255, 174, 111, 44, 69, 72, 41, 211, 3], OperandSize::Dword)
}

#[test]
fn vmovdqu16_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 65, 255, 170, 111, 235], OperandSize::Qword)
}

#[test]
fn vmovdqu16_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(YMM16)), operand2: Some(IndirectScaledIndexed(RAX, RSI, Four, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 255, 172, 111, 4, 176], OperandSize::Qword)
}

#[test]
fn vmovdqu16_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 255, 205, 111, 239], OperandSize::Dword)
}

#[test]
fn vmovdqu16_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Four, 1086699014, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 255, 202, 111, 180, 147, 6, 182, 197, 64], OperandSize::Dword)
}

#[test]
fn vmovdqu16_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 33, 255, 204, 111, 206], OperandSize::Qword)
}

#[test]
fn vmovdqu16_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(ZMM25)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Four, 1424003265, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 255, 201, 111, 140, 144, 193, 144, 224, 84], OperandSize::Qword)
}

#[test]
fn vmovdqu16_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 255, 141, 111, 195], OperandSize::Dword)
}

#[test]
fn vmovdqu16_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(IndirectScaledIndexed(EDX, EDX, Four, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 255, 8, 127, 20, 146], OperandSize::Dword)
}

#[test]
fn vmovdqu16_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 193, 255, 142, 111, 216], OperandSize::Qword)
}

#[test]
fn vmovdqu16_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(IndirectScaledIndexed(RAX, RDI, Eight, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 255, 8, 127, 12, 248], OperandSize::Qword)
}

#[test]
fn vmovdqu16_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 255, 172, 111, 197], OperandSize::Dword)
}

#[test]
fn vmovdqu16_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(IndirectScaledDisplaced(EDI, Eight, 1642308125, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 255, 40, 127, 44, 253, 29, 162, 227, 97], OperandSize::Dword)
}

#[test]
fn vmovdqu16_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 193, 255, 170, 111, 193], OperandSize::Qword)
}

#[test]
fn vmovdqu16_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(IndirectScaledDisplaced(RDX, Eight, 538044710, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 255, 40, 127, 36, 213, 38, 233, 17, 32], OperandSize::Qword)
}

#[test]
fn vmovdqu16_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 255, 204, 111, 232], OperandSize::Dword)
}

#[test]
fn vmovdqu16_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(IndirectScaledIndexed(ESI, EAX, Four, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 255, 72, 127, 52, 134], OperandSize::Dword)
}

#[test]
fn vmovdqu16_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 255, 203, 111, 198], OperandSize::Qword)
}

#[test]
fn vmovdqu16_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Indirect(RSI, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 255, 72, 127, 6], OperandSize::Qword)
}

