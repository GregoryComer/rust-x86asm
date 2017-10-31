use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vminps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 216, 93, 253], OperandSize::Dword)
}

#[test]
fn vminps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 240, 93, 63], OperandSize::Dword)
}

#[test]
fn vminps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 224, 93, 192], OperandSize::Qword)
}

#[test]
fn vminps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(RDX, 127763893, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 208, 93, 130, 181, 133, 157, 7], OperandSize::Qword)
}

#[test]
fn vminps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 212, 93, 220], OperandSize::Dword)
}

#[test]
fn vminps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(ESI, 449371033, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 220, 93, 174, 153, 219, 200, 26], OperandSize::Dword)
}

#[test]
fn vminps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 244, 93, 226], OperandSize::Qword)
}

#[test]
fn vminps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(RCX, RBX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 204, 93, 20, 89], OperandSize::Qword)
}

#[test]
fn vminps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 108, 142, 93, 246], OperandSize::Dword)
}

#[test]
fn vminps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 100, 141, 93, 3], OperandSize::Dword)
}

#[test]
fn vminps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 68, 155, 93, 35], OperandSize::Dword)
}

#[test]
fn vminps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM12)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 28, 142, 93, 194], OperandSize::Qword)
}

#[test]
fn vminps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM25)), operand3: Some(IndirectScaledIndexed(RCX, RCX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 52, 133, 93, 52, 137], OperandSize::Qword)
}

#[test]
fn vminps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM19)), operand3: Some(IndirectDisplaced(RBX, 86468216, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 100, 145, 93, 163, 120, 102, 39, 5], OperandSize::Qword)
}

#[test]
fn vminps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 108, 172, 93, 202], OperandSize::Dword)
}

#[test]
fn vminps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 1135870954, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 100, 170, 93, 12, 221, 234, 3, 180, 67], OperandSize::Dword)
}

#[test]
fn vminps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 100, 191, 93, 0], OperandSize::Dword)
}

#[test]
fn vminps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 145, 116, 173, 93, 249], OperandSize::Qword)
}

#[test]
fn vminps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM12)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 537289822, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 28, 172, 93, 52, 205, 94, 100, 6, 32], OperandSize::Qword)
}

#[test]
fn vminps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM11)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 1172725205, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 97, 36, 186, 93, 20, 157, 213, 93, 230, 69], OperandSize::Qword)
}

#[test]
fn vminps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 92, 158, 93, 239], OperandSize::Dword)
}

#[test]
fn vminps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexed(EBX, ESI, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 68, 205, 93, 60, 179], OperandSize::Dword)
}

#[test]
fn vminps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM0)), operand3: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 124, 218, 93, 1], OperandSize::Dword)
}

#[test]
fn vminps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM10)), operand3: Some(Direct(ZMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 193, 44, 158, 93, 228], OperandSize::Qword)
}

#[test]
fn vminps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM12)), operand3: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 28, 204, 93, 50], OperandSize::Qword)
}

#[test]
fn vminps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectDisplaced(RCX, 676470096, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 97, 100, 223, 93, 153, 80, 29, 82, 40], OperandSize::Qword)
}

