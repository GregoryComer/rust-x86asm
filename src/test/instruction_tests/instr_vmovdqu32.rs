use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovdqu32_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 126, 137, 111, 203], OperandSize::Dword)
}

#[test]
fn vmovdqu32_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(EDX, 352492477, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 126, 143, 111, 130, 189, 155, 2, 21], OperandSize::Dword)
}

#[test]
fn vmovdqu32_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 126, 142, 111, 205], OperandSize::Qword)
}

#[test]
fn vmovdqu32_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(XMM14)), operand2: Some(IndirectScaledIndexed(RDX, RBX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 126, 142, 111, 52, 218], OperandSize::Qword)
}

#[test]
fn vmovdqu32_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 126, 174, 111, 221], OperandSize::Dword)
}

#[test]
fn vmovdqu32_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(YMM2)), operand2: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 126, 169, 111, 17], OperandSize::Dword)
}

#[test]
fn vmovdqu32_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 33, 126, 174, 111, 207], OperandSize::Qword)
}

#[test]
fn vmovdqu32_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(YMM28)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 1023868237, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 126, 169, 111, 36, 205, 77, 253, 6, 61], OperandSize::Qword)
}

#[test]
fn vmovdqu32_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 126, 202, 111, 192], OperandSize::Dword)
}

#[test]
fn vmovdqu32_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 740973565, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 126, 202, 111, 28, 181, 253, 91, 42, 44], OperandSize::Dword)
}

#[test]
fn vmovdqu32_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 65, 126, 201, 111, 224], OperandSize::Qword)
}

#[test]
fn vmovdqu32_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(ZMM21)), operand2: Some(Indirect(RAX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 126, 202, 111, 40], OperandSize::Qword)
}

#[test]
fn vmovdqu32_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 126, 138, 111, 197], OperandSize::Dword)
}

#[test]
fn vmovdqu32_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 126, 8, 127, 57], OperandSize::Dword)
}

#[test]
fn vmovdqu32_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 126, 142, 111, 205], OperandSize::Qword)
}

#[test]
fn vmovdqu32_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(IndirectDisplaced(RDI, 1979312060, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 126, 8, 127, 159, 188, 231, 249, 117], OperandSize::Qword)
}

#[test]
fn vmovdqu32_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 126, 172, 111, 213], OperandSize::Dword)
}

#[test]
fn vmovdqu32_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Four, 1540571722, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 126, 40, 127, 140, 184, 74, 66, 211, 91], OperandSize::Dword)
}

#[test]
fn vmovdqu32_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 177, 126, 174, 111, 241], OperandSize::Qword)
}

#[test]
fn vmovdqu32_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 113, 126, 40, 127, 31], OperandSize::Qword)
}

#[test]
fn vmovdqu32_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 126, 203, 111, 219], OperandSize::Dword)
}

#[test]
fn vmovdqu32_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Eight, 1349585882, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 126, 72, 127, 156, 243, 218, 11, 113, 80], OperandSize::Dword)
}

#[test]
fn vmovdqu32_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 126, 201, 111, 192], OperandSize::Qword)
}

#[test]
fn vmovdqu32_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Indirect(RBX, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 126, 72, 127, 11], OperandSize::Qword)
}

