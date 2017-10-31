use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vprorvd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 85, 139, 20, 237], OperandSize::Dword)
}

#[test]
fn vprorvd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 93, 138, 20, 27], OperandSize::Dword)
}

#[test]
fn vprorvd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(EAX, 518744497, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 101, 156, 20, 144, 177, 105, 235, 30], OperandSize::Dword)
}

#[test]
fn vprorvd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM13)), operand3: Some(Direct(XMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 50, 21, 143, 20, 237], OperandSize::Qword)
}

#[test]
fn vprorvd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(RDX, 529204212, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 101, 143, 20, 130, 244, 3, 139, 31], OperandSize::Qword)
}

#[test]
fn vprorvd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM18)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Two, 1474021169, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 109, 149, 20, 156, 91, 49, 199, 219, 87], OperandSize::Qword)
}

#[test]
fn vprorvd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 117, 174, 20, 242], OperandSize::Dword)
}

#[test]
fn vprorvd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Two, 2015403673, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 77, 171, 20, 180, 91, 153, 158, 32, 120], OperandSize::Dword)
}

#[test]
fn vprorvd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Two, 2097233301, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 109, 191, 20, 164, 119, 149, 61, 1, 125], OperandSize::Dword)
}

#[test]
fn vprorvd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM18)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 109, 161, 20, 228], OperandSize::Qword)
}

#[test]
fn vprorvd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(RDI, RCX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 109, 172, 20, 36, 207], OperandSize::Qword)
}

#[test]
fn vprorvd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM31)), operand3: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 5, 180, 20, 42], OperandSize::Qword)
}

#[test]
fn vprorvd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 69, 206, 20, 246], OperandSize::Dword)
}

#[test]
fn vprorvd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM5)), operand3: Some(Indirect(EBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 85, 202, 20, 19], OperandSize::Dword)
}

#[test]
fn vprorvd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectDisplaced(EBX, 1785187895, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 69, 220, 20, 139, 55, 206, 103, 106], OperandSize::Dword)
}

#[test]
fn vprorvd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 101, 205, 20, 216], OperandSize::Qword)
}

#[test]
fn vprorvd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM27)), operand3: Some(Indirect(RSI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 37, 193, 20, 62], OperandSize::Qword)
}

#[test]
fn vprorvd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexed(RSI, RCX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 226, 101, 220, 20, 36, 206], OperandSize::Qword)
}

