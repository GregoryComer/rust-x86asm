use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovaps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 40, 234], OperandSize::Dword)
}

#[test]
fn vmovaps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(EBX, ECX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 40, 4, 139], OperandSize::Dword)
}

#[test]
fn vmovaps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 40, 204], OperandSize::Qword)
}

#[test]
fn vmovaps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 1447885844, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 40, 4, 117, 20, 252, 76, 86], OperandSize::Qword)
}

#[test]
fn vmovaps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 40, 226], OperandSize::Dword)
}

#[test]
fn vmovaps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 1224542350, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 40, 20, 125, 142, 8, 253, 72], OperandSize::Dword)
}

#[test]
fn vmovaps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 40, 253], OperandSize::Qword)
}

#[test]
fn vmovaps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(YMM0)), operand2: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 40, 7], OperandSize::Qword)
}

#[test]
fn vmovaps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 124, 143, 40, 213], OperandSize::Dword)
}

#[test]
fn vmovaps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 2023995000, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 124, 143, 40, 60, 69, 120, 182, 163, 120], OperandSize::Dword)
}

#[test]
fn vmovaps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 209, 124, 139, 40, 237], OperandSize::Qword)
}

#[test]
fn vmovaps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(XMM11)), operand2: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 124, 141, 40, 24], OperandSize::Qword)
}

#[test]
fn vmovaps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 124, 170, 40, 204], OperandSize::Dword)
}

#[test]
fn vmovaps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexed(EAX, EBX, Four, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 124, 170, 40, 20, 152], OperandSize::Dword)
}

#[test]
fn vmovaps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 177, 124, 175, 40, 248], OperandSize::Qword)
}

#[test]
fn vmovaps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(YMM13)), operand2: Some(IndirectScaledIndexed(RSI, RCX, Eight, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 124, 169, 40, 44, 206], OperandSize::Qword)
}

#[test]
fn vmovaps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 124, 201, 40, 236], OperandSize::Dword)
}

#[test]
fn vmovaps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Four, 487622766, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 124, 201, 40, 172, 151, 110, 136, 16, 29], OperandSize::Dword)
}

#[test]
fn vmovaps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 193, 124, 201, 40, 205], OperandSize::Qword)
}

#[test]
fn vmovaps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(ZMM24)), operand2: Some(IndirectScaledIndexed(RCX, RDI, Eight, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 124, 202, 40, 4, 249], OperandSize::Qword)
}

#[test]
fn vmovaps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 40, 255], OperandSize::Dword)
}

#[test]
fn vmovaps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(IndirectScaledIndexed(ESI, EDI, Eight, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 41, 36, 254], OperandSize::Dword)
}

#[test]
fn vmovaps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 40, 197], OperandSize::Qword)
}

#[test]
fn vmovaps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(IndirectScaledIndexed(RDI, RDI, Two, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 41, 52, 127], OperandSize::Qword)
}

#[test]
fn vmovaps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 40, 225], OperandSize::Dword)
}

#[test]
fn vmovaps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(IndirectDisplaced(EDX, 527055242, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 41, 130, 138, 57, 106, 31], OperandSize::Dword)
}

#[test]
fn vmovaps_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 40, 220], OperandSize::Qword)
}

#[test]
fn vmovaps_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Four, 974771527, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 41, 156, 146, 71, 213, 25, 58], OperandSize::Qword)
}

#[test]
fn vmovaps_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 124, 142, 40, 246], OperandSize::Dword)
}

#[test]
fn vmovaps_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 41, 0], OperandSize::Dword)
}

#[test]
fn vmovaps_31() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 49, 124, 143, 40, 220], OperandSize::Qword)
}

#[test]
fn vmovaps_32() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Four, 2111561293, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 120, 41, 164, 146, 77, 222, 219, 125], OperandSize::Qword)
}

#[test]
fn vmovaps_33() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 124, 172, 40, 221], OperandSize::Dword)
}

#[test]
fn vmovaps_34() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(IndirectScaledDisplaced(ECX, Four, 873856627, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 41, 12, 141, 115, 254, 21, 52], OperandSize::Dword)
}

#[test]
fn vmovaps_35() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 49, 124, 170, 40, 248], OperandSize::Qword)
}

#[test]
fn vmovaps_36() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(IndirectScaledDisplaced(RSI, Four, 1105480038, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 41, 12, 181, 102, 73, 228, 65], OperandSize::Qword)
}

#[test]
fn vmovaps_37() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 124, 207, 40, 207], OperandSize::Dword)
}

#[test]
fn vmovaps_38() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(IndirectDisplaced(EAX, 1020267894, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 124, 72, 41, 184, 118, 13, 208, 60], OperandSize::Dword)
}

#[test]
fn vmovaps_39() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 124, 204, 40, 222], OperandSize::Qword)
}

#[test]
fn vmovaps_40() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(IndirectDisplaced(RSI, 2083169281, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 124, 72, 41, 150, 1, 164, 42, 124], OperandSize::Qword)
}

