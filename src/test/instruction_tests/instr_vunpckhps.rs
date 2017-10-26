use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vunpckhps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 216, 21, 225], OperandSize::Dword)
}

#[test]
fn vunpckhps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Eight, 1391018198, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 232, 21, 164, 200, 214, 64, 233, 82], OperandSize::Dword)
}

#[test]
fn vunpckhps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 192, 21, 219], OperandSize::Qword)
}

#[test]
fn vunpckhps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 954890432, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 200, 21, 60, 77, 192, 120, 234, 56], OperandSize::Qword)
}

#[test]
fn vunpckhps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 220, 21, 203], OperandSize::Dword)
}

#[test]
fn vunpckhps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 193582182, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 196, 21, 60, 117, 102, 212, 137, 11], OperandSize::Dword)
}

#[test]
fn vunpckhps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 244, 21, 206], OperandSize::Qword)
}

#[test]
fn vunpckhps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Eight, 1257180535, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 21, 188, 217, 119, 13, 239, 74], OperandSize::Qword)
}

#[test]
fn vunpckhps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 100, 140, 21, 236], OperandSize::Dword)
}

#[test]
fn vunpckhps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 785545054, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 84, 138, 21, 12, 77, 94, 119, 210, 46], OperandSize::Dword)
}

#[test]
fn vunpckhps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 54119003, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 76, 159, 21, 52, 77, 91, 202, 57, 3], OperandSize::Dword)
}

#[test]
fn vunpckhps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM26)), operand3: Some(Direct(XMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 1, 44, 134, 21, 209], OperandSize::Qword)
}

#[test]
fn vunpckhps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM19)), operand3: Some(IndirectScaledIndexed(RDX, RSI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 100, 131, 21, 60, 114], OperandSize::Qword)
}

#[test]
fn vunpckhps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM19)), operand3: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 113, 100, 150, 21, 24], OperandSize::Qword)
}

#[test]
fn vunpckhps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 76, 172, 21, 221], OperandSize::Dword)
}

#[test]
fn vunpckhps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 92, 172, 21, 23], OperandSize::Dword)
}

#[test]
fn vunpckhps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(ECX, EAX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 76, 189, 21, 20, 65], OperandSize::Dword)
}

#[test]
fn vunpckhps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM30)), operand3: Some(Direct(YMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 81, 12, 163, 21, 220], OperandSize::Qword)
}

#[test]
fn vunpckhps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM31)), operand3: Some(IndirectDisplaced(RDI, 1454131471, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 4, 161, 21, 135, 15, 73, 172, 86], OperandSize::Qword)
}

#[test]
fn vunpckhps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Four, 367542483, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 124, 188, 21, 148, 152, 211, 64, 232, 21], OperandSize::Qword)
}

#[test]
fn vunpckhps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 76, 202, 21, 253], OperandSize::Dword)
}

#[test]
fn vunpckhps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectDisplaced(ECX, 809947407, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 68, 204, 21, 185, 15, 209, 70, 48], OperandSize::Dword)
}

#[test]
fn vunpckhps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectDisplaced(EAX, 1743400859, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 92, 220, 21, 176, 155, 47, 234, 103], OperandSize::Dword)
}

#[test]
fn vunpckhps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 81, 76, 207, 21, 238], OperandSize::Qword)
}

#[test]
fn vunpckhps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM8)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Eight, 1116000852, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 60, 207, 21, 156, 217, 84, 210, 132, 66], OperandSize::Qword)
}

#[test]
fn vunpckhps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM24)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Four, 907089346, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 60, 215, 21, 140, 136, 194, 21, 17, 54], OperandSize::Qword)
}

