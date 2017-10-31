use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovdqu32_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 126, 137, 111, 227], OperandSize::Dword)
}

#[test]
fn vmovdqu32_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 126, 139, 111, 6], OperandSize::Dword)
}

#[test]
fn vmovdqu32_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 126, 137, 111, 254], OperandSize::Qword)
}

#[test]
fn vmovdqu32_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Eight, 441892109, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 126, 143, 111, 140, 223, 13, 189, 86, 26], OperandSize::Qword)
}

#[test]
fn vmovdqu32_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 126, 172, 111, 194], OperandSize::Dword)
}

#[test]
fn vmovdqu32_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 814442407, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 126, 170, 111, 28, 149, 167, 103, 139, 48], OperandSize::Dword)
}

#[test]
fn vmovdqu32_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 193, 126, 174, 111, 211], OperandSize::Qword)
}

#[test]
fn vmovdqu32_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(YMM19)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Eight, 1742217039, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 126, 173, 111, 156, 219, 79, 31, 216, 103], OperandSize::Qword)
}

#[test]
fn vmovdqu32_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 126, 201, 111, 221], OperandSize::Dword)
}

#[test]
fn vmovdqu32_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledIndexed(EDI, ESI, Four, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 126, 207, 111, 28, 183], OperandSize::Dword)
}

#[test]
fn vmovdqu32_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 193, 126, 204, 111, 217], OperandSize::Qword)
}

#[test]
fn vmovdqu32_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(ZMM15)), operand2: Some(IndirectDisplaced(RCX, 1718447371, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 126, 201, 111, 185, 11, 109, 109, 102], OperandSize::Qword)
}

#[test]
fn vmovdqu32_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 126, 141, 111, 194], OperandSize::Dword)
}

#[test]
fn vmovdqu32_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(IndirectScaledIndexed(EBX, EDI, Two, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 126, 8, 127, 60, 123], OperandSize::Dword)
}

#[test]
fn vmovdqu32_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 126, 143, 111, 198], OperandSize::Qword)
}

#[test]
fn vmovdqu32_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Four, 1994345599, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 113, 126, 8, 127, 172, 177, 127, 76, 223, 118], OperandSize::Qword)
}

#[test]
fn vmovdqu32_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 126, 171, 111, 211], OperandSize::Dword)
}

#[test]
fn vmovdqu32_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Two, 131029348, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 126, 40, 127, 132, 114, 100, 89, 207, 7], OperandSize::Dword)
}

#[test]
fn vmovdqu32_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 126, 171, 111, 204], OperandSize::Qword)
}

#[test]
fn vmovdqu32_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Four, 39168481, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 126, 40, 127, 140, 184, 225, 169, 85, 2], OperandSize::Qword)
}

#[test]
fn vmovdqu32_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 126, 205, 111, 209], OperandSize::Dword)
}

#[test]
fn vmovdqu32_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(IndirectScaledIndexed(EAX, EAX, Four, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 126, 72, 127, 52, 128], OperandSize::Dword)
}

#[test]
fn vmovdqu32_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 145, 126, 202, 111, 204], OperandSize::Qword)
}

#[test]
fn vmovdqu32_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Two, 239705495, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 126, 72, 127, 156, 79, 151, 157, 73, 14], OperandSize::Qword)
}

