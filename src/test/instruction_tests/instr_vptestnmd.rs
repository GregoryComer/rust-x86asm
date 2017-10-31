use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vptestnmd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 118, 15, 39, 246], OperandSize::Dword)
}

#[test]
fn vptestnmd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(EDX, 6611175, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 102, 9, 39, 162, 231, 224, 100, 0], OperandSize::Dword)
}

#[test]
fn vptestnmd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(EDI, 101724745, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 126, 30, 39, 191, 73, 50, 16, 6], OperandSize::Dword)
}

#[test]
fn vptestnmd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM21)), operand3: Some(Direct(XMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 210, 86, 4, 39, 228], OperandSize::Qword)
}

#[test]
fn vptestnmd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 1081876162, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 126, 11, 39, 28, 157, 194, 30, 124, 64], OperandSize::Qword)
}

#[test]
fn vptestnmd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM26)), operand3: Some(IndirectDisplaced(RSI, 1663946662, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 46, 21, 39, 142, 166, 207, 45, 99], OperandSize::Qword)
}

#[test]
fn vptestnmd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 118, 47, 39, 225], OperandSize::Dword)
}

#[test]
fn vptestnmd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 535518604, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 110, 47, 39, 44, 197, 140, 93, 235, 31], OperandSize::Dword)
}

#[test]
fn vptestnmd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Four, 967251147, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 86, 60, 39, 148, 150, 203, 20, 167, 57], OperandSize::Dword)
}

#[test]
fn vptestnmd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM15)), operand3: Some(Direct(YMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 178, 6, 44, 39, 212], OperandSize::Qword)
}

#[test]
fn vptestnmd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(RDX, RBX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 118, 46, 39, 52, 154], OperandSize::Qword)
}

#[test]
fn vptestnmd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM12)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Two, 1552970738, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 30, 59, 39, 164, 91, 242, 115, 144, 92], OperandSize::Qword)
}

#[test]
fn vptestnmd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 118, 74, 39, 245], OperandSize::Dword)
}

#[test]
fn vptestnmd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 1828052509, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 126, 78, 39, 20, 197, 29, 222, 245, 108], OperandSize::Dword)
}

#[test]
fn vptestnmd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Four, 1602199567, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 78, 92, 39, 172, 182, 15, 160, 127, 95], OperandSize::Dword)
}

#[test]
fn vptestnmd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM30)), operand3: Some(Direct(ZMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 146, 14, 67, 39, 252], OperandSize::Qword)
}

#[test]
fn vptestnmd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Four, 2100887183, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 102, 79, 39, 188, 134, 143, 254, 56, 125], OperandSize::Qword)
}

#[test]
fn vptestnmd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM20)), operand3: Some(IndirectDisplaced(RDI, 451228126, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 94, 85, 39, 167, 222, 49, 229, 26], OperandSize::Qword)
}

