use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsubadd213ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 167, 218], OperandSize::Dword)
}

#[test]
fn vfmsubadd213ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(EDX, EBX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 167, 44, 218], OperandSize::Dword)
}

#[test]
fn vfmsubadd213ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 167, 194], OperandSize::Qword)
}

#[test]
fn vfmsubadd213ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Four, 889431770, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 167, 140, 143, 218, 166, 3, 53], OperandSize::Qword)
}

#[test]
fn vfmsubadd213ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 167, 226], OperandSize::Dword)
}

#[test]
fn vfmsubadd213ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Four, 324436159, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 167, 140, 190, 191, 128, 86, 19], OperandSize::Dword)
}

#[test]
fn vfmsubadd213ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 167, 255], OperandSize::Qword)
}

#[test]
fn vfmsubadd213ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Two, 1720809287, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 167, 164, 114, 71, 119, 145, 102], OperandSize::Qword)
}

#[test]
fn vfmsubadd213ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 117, 138, 167, 217], OperandSize::Dword)
}

#[test]
fn vfmsubadd213ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 1250235010, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 85, 139, 167, 36, 197, 130, 18, 133, 74], OperandSize::Dword)
}

#[test]
fn vfmsubadd213ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 628053055, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 101, 158, 167, 52, 181, 63, 84, 111, 37], OperandSize::Dword)
}

#[test]
fn vfmsubadd213ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM8)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 61, 138, 167, 253], OperandSize::Qword)
}

#[test]
fn vfmsubadd213ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM14)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 13, 137, 167, 31], OperandSize::Qword)
}

#[test]
fn vfmsubadd213ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(RAX, 588949961, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 69, 155, 167, 144, 201, 169, 26, 35], OperandSize::Qword)
}

#[test]
fn vfmsubadd213ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 109, 170, 167, 239], OperandSize::Dword)
}

#[test]
fn vfmsubadd213ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(EDX, 620541761, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 93, 175, 167, 138, 65, 183, 252, 36], OperandSize::Dword)
}

#[test]
fn vfmsubadd213ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(ECX, EDX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 190, 167, 44, 81], OperandSize::Dword)
}

#[test]
fn vfmsubadd213ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM17)), operand3: Some(Direct(YMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 194, 117, 166, 167, 211], OperandSize::Qword)
}

#[test]
fn vfmsubadd213ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM8)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 1526797602, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 61, 174, 167, 52, 141, 34, 21, 1, 91], OperandSize::Qword)
}

#[test]
fn vfmsubadd213ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM27)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 1674441736, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 37, 183, 167, 12, 77, 8, 244, 205, 99], OperandSize::Qword)
}

#[test]
fn vfmsubadd213ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 85, 251, 167, 197], OperandSize::Dword)
}

#[test]
fn vfmsubadd213ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 1066183740, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 69, 201, 167, 4, 117, 60, 172, 140, 63], OperandSize::Dword)
}

#[test]
fn vfmsubadd213ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexed(ESI, EDX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 77, 220, 167, 60, 86], OperandSize::Dword)
}

#[test]
fn vfmsubadd213ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM21)), operand3: Some(Direct(ZMM14)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 66, 85, 212, 167, 230], OperandSize::Qword)
}

#[test]
fn vfmsubadd213ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM1)), operand3: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 117, 206, 167, 50], OperandSize::Qword)
}

#[test]
fn vfmsubadd213ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM20)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Eight, 1689476544, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 93, 211, 167, 172, 242, 192, 93, 179, 100], OperandSize::Qword)
}

