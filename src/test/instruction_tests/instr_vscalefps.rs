use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vscalefps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 101, 141, 44, 255], OperandSize::Dword)
}

#[test]
fn vscalefps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(ESI, EBX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 101, 137, 44, 52, 158], OperandSize::Dword)
}

#[test]
fn vscalefps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 69, 155, 44, 32], OperandSize::Dword)
}

#[test]
fn vscalefps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM27)), operand3: Some(Direct(XMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 82, 37, 129, 44, 228], OperandSize::Qword)
}

#[test]
fn vscalefps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM11)), operand3: Some(IndirectDisplaced(RDI, 821789981, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 37, 137, 44, 191, 29, 133, 251, 48], OperandSize::Qword)
}

#[test]
fn vscalefps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM23)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 175356951, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 69, 148, 44, 44, 197, 23, 188, 115, 10], OperandSize::Qword)
}

#[test]
fn vscalefps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 85, 170, 44, 201], OperandSize::Dword)
}

#[test]
fn vscalefps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Four, 256152810, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 117, 172, 44, 172, 182, 234, 148, 68, 15], OperandSize::Dword)
}

#[test]
fn vscalefps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 1521763454, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 117, 186, 44, 12, 77, 126, 68, 180, 90], OperandSize::Dword)
}

#[test]
fn vscalefps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM10)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 45, 174, 44, 222], OperandSize::Qword)
}

#[test]
fn vscalefps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM10)), operand3: Some(IndirectScaledIndexed(RAX, RDX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 45, 175, 44, 52, 144], OperandSize::Qword)
}

#[test]
fn vscalefps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(RAX, RDX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 125, 190, 44, 20, 144], OperandSize::Qword)
}

#[test]
fn vscalefps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 77, 188, 44, 246], OperandSize::Dword)
}

#[test]
fn vscalefps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 1311076305, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 205, 44, 44, 69, 209, 111, 37, 78], OperandSize::Dword)
}

#[test]
fn vscalefps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM6)), operand3: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 77, 219, 44, 16], OperandSize::Dword)
}

#[test]
fn vscalefps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM27)), operand3: Some(Direct(ZMM25)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 18, 37, 241, 44, 225], OperandSize::Qword)
}

#[test]
fn vscalefps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM9)), operand3: Some(Indirect(RBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 53, 204, 44, 11], OperandSize::Qword)
}

#[test]
fn vscalefps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM9)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 1820979451, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 53, 223, 44, 4, 157, 251, 240, 137, 108], OperandSize::Qword)
}

