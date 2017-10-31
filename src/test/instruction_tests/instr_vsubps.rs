use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vsubps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 224, 92, 230], OperandSize::Dword)
}

#[test]
fn vsubps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 216, 92, 32], OperandSize::Dword)
}

#[test]
fn vsubps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 224, 92, 252], OperandSize::Qword)
}

#[test]
fn vsubps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 208, 92, 40], OperandSize::Qword)
}

#[test]
fn vsubps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 220, 92, 251], OperandSize::Dword)
}

#[test]
fn vsubps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(ESI, ECX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 204, 92, 4, 78], OperandSize::Dword)
}

#[test]
fn vsubps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 244, 92, 221], OperandSize::Qword)
}

#[test]
fn vsubps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Eight, 1595840383, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 228, 92, 164, 250, 127, 151, 30, 95], OperandSize::Qword)
}

#[test]
fn vsubps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 124, 139, 92, 225], OperandSize::Dword)
}

#[test]
fn vsubps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 2025672814, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 124, 138, 92, 44, 253, 110, 80, 189, 120], OperandSize::Dword)
}

#[test]
fn vsubps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Four, 738075767, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 108, 154, 92, 156, 147, 119, 36, 254, 43], OperandSize::Dword)
}

#[test]
fn vsubps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM13)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 20, 143, 92, 195], OperandSize::Qword)
}

#[test]
fn vsubps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM29)), operand3: Some(IndirectDisplaced(RBX, 1515222823, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 20, 133, 92, 131, 39, 119, 80, 90], OperandSize::Qword)
}

#[test]
fn vsubps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM9)), operand3: Some(IndirectDisplaced(RDI, 2098777099, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 225, 52, 153, 92, 183, 11, 204, 24, 125], OperandSize::Qword)
}

#[test]
fn vsubps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 76, 170, 92, 253], OperandSize::Dword)
}

#[test]
fn vsubps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 116, 171, 92, 55], OperandSize::Dword)
}

#[test]
fn vsubps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(EAX, ESI, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 116, 185, 92, 12, 240], OperandSize::Dword)
}

#[test]
fn vsubps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM14)), operand3: Some(Direct(YMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 161, 12, 175, 92, 228], OperandSize::Qword)
}

#[test]
fn vsubps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM19)), operand3: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 100, 163, 92, 35], OperandSize::Qword)
}

#[test]
fn vsubps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM19)), operand3: Some(IndirectDisplaced(RDX, 1505566601, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 100, 178, 92, 154, 137, 31, 189, 89], OperandSize::Qword)
}

#[test]
fn vsubps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 92, 185, 92, 206], OperandSize::Dword)
}

#[test]
fn vsubps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 335256498, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 76, 203, 92, 20, 125, 178, 155, 251, 19], OperandSize::Dword)
}

#[test]
fn vsubps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectDisplaced(EBX, 838322972, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 76, 218, 92, 155, 28, 203, 247, 49], OperandSize::Dword)
}

#[test]
fn vsubps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM9)), operand3: Some(Direct(ZMM27)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 129, 52, 157, 92, 227], OperandSize::Qword)
}

#[test]
fn vsubps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectDisplaced(RDX, 1497429208, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 92, 202, 92, 138, 216, 244, 64, 89], OperandSize::Qword)
}

#[test]
fn vsubps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM14)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Two, 1883727988, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 12, 218, 92, 172, 120, 116, 104, 71, 112], OperandSize::Qword)
}

