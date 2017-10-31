use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovaps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 40, 222], OperandSize::Dword)
}

#[test]
fn vmovaps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Eight, 1428444403, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 40, 132, 219, 243, 84, 36, 85], OperandSize::Dword)
}

#[test]
fn vmovaps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 40, 253], OperandSize::Qword)
}

#[test]
fn vmovaps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(RCX, RBX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 40, 52, 89], OperandSize::Qword)
}

#[test]
fn vmovaps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 40, 239], OperandSize::Dword)
}

#[test]
fn vmovaps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(YMM6)), operand2: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 40, 48], OperandSize::Dword)
}

#[test]
fn vmovaps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 40, 231], OperandSize::Qword)
}

#[test]
fn vmovaps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(YMM0)), operand2: Some(IndirectDisplaced(RDX, 692080843, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 40, 130, 203, 80, 64, 41], OperandSize::Qword)
}

#[test]
fn vmovaps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 124, 141, 40, 210], OperandSize::Dword)
}

#[test]
fn vmovaps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Four, 326063444, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 124, 142, 40, 148, 128, 84, 85, 111, 19], OperandSize::Dword)
}

#[test]
fn vmovaps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 193, 124, 138, 40, 251], OperandSize::Qword)
}

#[test]
fn vmovaps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(RAX, 98842518, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 124, 141, 40, 160, 150, 55, 228, 5], OperandSize::Qword)
}

#[test]
fn vmovaps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 124, 171, 40, 232], OperandSize::Dword)
}

#[test]
fn vmovaps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(YMM1)), operand2: Some(IndirectDisplaced(EDX, 1809526523, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 124, 174, 40, 138, 251, 46, 219, 107], OperandSize::Dword)
}

#[test]
fn vmovaps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 209, 124, 171, 40, 238], OperandSize::Qword)
}

#[test]
fn vmovaps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(YMM1)), operand2: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 124, 171, 40, 15], OperandSize::Qword)
}

#[test]
fn vmovaps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 124, 205, 40, 242], OperandSize::Dword)
}

#[test]
fn vmovaps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledDisplaced(EBX, Four, 361783783, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 124, 201, 40, 44, 157, 231, 97, 144, 21], OperandSize::Dword)
}

#[test]
fn vmovaps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 1, 124, 203, 40, 197], OperandSize::Qword)
}

#[test]
fn vmovaps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(ZMM8)), operand2: Some(IndirectScaledIndexed(RDX, RBX, Eight, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 124, 202, 40, 4, 218], OperandSize::Qword)
}

#[test]
fn vmovaps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 40, 221], OperandSize::Dword)
}

#[test]
fn vmovaps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 41, 18], OperandSize::Dword)
}

#[test]
fn vmovaps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 40, 200], OperandSize::Qword)
}

#[test]
fn vmovaps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(IndirectScaledDisplaced(RDX, Two, 1951021031, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 41, 36, 85, 231, 55, 74, 116], OperandSize::Qword)
}

#[test]
fn vmovaps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 40, 223], OperandSize::Dword)
}

#[test]
fn vmovaps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Eight, 1419661266, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 41, 132, 248, 210, 79, 158, 84], OperandSize::Dword)
}

#[test]
fn vmovaps_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 40, 225], OperandSize::Qword)
}

#[test]
fn vmovaps_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(IndirectScaledDisplaced(RDX, Four, 1949103437, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 41, 12, 149, 77, 245, 44, 116], OperandSize::Qword)
}

#[test]
fn vmovaps_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 124, 137, 40, 238], OperandSize::Dword)
}

#[test]
fn vmovaps_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(IndirectDisplaced(EBX, 1142968551, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 41, 163, 231, 80, 32, 68], OperandSize::Dword)
}

#[test]
fn vmovaps_31() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 49, 124, 143, 40, 199], OperandSize::Qword)
}

#[test]
fn vmovaps_32() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(IndirectScaledIndexed(RBX, RCX, Four, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 124, 8, 41, 60, 139], OperandSize::Qword)
}

#[test]
fn vmovaps_33() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 124, 172, 40, 239], OperandSize::Dword)
}

#[test]
fn vmovaps_34() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Eight, 1023956662, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 41, 132, 215, 182, 86, 8, 61], OperandSize::Dword)
}

#[test]
fn vmovaps_35() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 65, 124, 172, 40, 255], OperandSize::Qword)
}

#[test]
fn vmovaps_36() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(IndirectScaledIndexed(RSI, RCX, Four, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 41, 52, 142], OperandSize::Qword)
}

#[test]
fn vmovaps_37() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 124, 206, 40, 255], OperandSize::Dword)
}

#[test]
fn vmovaps_38() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(IndirectDisplaced(EDX, 105137289, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 124, 72, 41, 162, 137, 68, 68, 6], OperandSize::Dword)
}

#[test]
fn vmovaps_39() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 124, 204, 40, 247], OperandSize::Qword)
}

#[test]
fn vmovaps_40() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(IndirectDisplaced(RCX, 905402604, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 124, 72, 41, 145, 236, 88, 247, 53], OperandSize::Qword)
}

