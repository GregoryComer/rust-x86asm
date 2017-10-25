use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vunpckhps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 224, 21, 200], OperandSize::Dword)
}

#[test]
fn vunpckhps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 224, 21, 57], OperandSize::Dword)
}

#[test]
fn vunpckhps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 21, 232], OperandSize::Qword)
}

#[test]
fn vunpckhps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Four, 909523892, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 192, 21, 180, 176, 180, 59, 54, 54], OperandSize::Qword)
}

#[test]
fn vunpckhps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 204, 21, 229], OperandSize::Dword)
}

#[test]
fn vunpckhps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 228, 21, 8], OperandSize::Dword)
}

#[test]
fn vunpckhps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 236, 21, 215], OperandSize::Qword)
}

#[test]
fn vunpckhps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(RDX, 1695541560, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 204, 21, 130, 56, 233, 15, 101], OperandSize::Qword)
}

#[test]
fn vunpckhps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 68, 138, 21, 243], OperandSize::Dword)
}

#[test]
fn vunpckhps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(ECX, 1534977051, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 116, 140, 21, 129, 27, 228, 125, 91], OperandSize::Dword)
}

#[test]
fn vunpckhps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 100, 158, 21, 42], OperandSize::Dword)
}

#[test]
fn vunpckhps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM9)), operand3: Some(Direct(XMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 209, 52, 140, 21, 226], OperandSize::Qword)
}

#[test]
fn vunpckhps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 30627622, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 100, 142, 21, 44, 133, 38, 87, 211, 1], OperandSize::Qword)
}

#[test]
fn vunpckhps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM17)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 711147771, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 116, 149, 21, 20, 133, 251, 64, 99, 42], OperandSize::Qword)
}

#[test]
fn vunpckhps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 68, 175, 21, 211], OperandSize::Dword)
}

#[test]
fn vunpckhps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(ESI, 1698583289, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 84, 175, 21, 174, 249, 82, 62, 101], OperandSize::Dword)
}

#[test]
fn vunpckhps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Two, 2055815169, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 116, 190, 21, 180, 126, 1, 64, 137, 122], OperandSize::Dword)
}

#[test]
fn vunpckhps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM20)), operand3: Some(Direct(YMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 33, 92, 164, 21, 206], OperandSize::Qword)
}

#[test]
fn vunpckhps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM11)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 719817842, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 36, 171, 21, 52, 245, 114, 140, 231, 42], OperandSize::Qword)
}

#[test]
fn vunpckhps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM18)), operand3: Some(IndirectScaledIndexed(RDI, RBX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 97, 108, 178, 21, 60, 159], OperandSize::Qword)
}

#[test]
fn vunpckhps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 92, 203, 21, 203], OperandSize::Dword)
}

#[test]
fn vunpckhps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectDisplaced(EDX, 1961781248, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 76, 204, 21, 178, 0, 104, 238, 116], OperandSize::Dword)
}

#[test]
fn vunpckhps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectDisplaced(ESI, 1486149870, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 84, 221, 21, 142, 238, 216, 148, 88], OperandSize::Dword)
}

#[test]
fn vunpckhps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM27)), operand3: Some(Direct(ZMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 129, 36, 194, 21, 216], OperandSize::Qword)
}

#[test]
fn vunpckhps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Four, 502365672, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 68, 202, 21, 180, 147, 232, 125, 241, 29], OperandSize::Qword)
}

#[test]
fn vunpckhps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 858577241, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 113, 84, 220, 21, 4, 93, 89, 217, 44, 51], OperandSize::Qword)
}

