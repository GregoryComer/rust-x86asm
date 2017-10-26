use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vreduceps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(71)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 125, 137, 86, 201, 71], OperandSize::Dword)
}

#[test]
fn vreduceps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(EAX, 355325484, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(71)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 125, 137, 86, 128, 44, 214, 45, 21, 71], OperandSize::Dword)
}

#[test]
fn vreduceps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(EBX, 509668120, Some(OperandSize::Dword), None)), operand3: Some(Literal8(60)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 125, 157, 86, 179, 24, 235, 96, 30, 60], OperandSize::Dword)
}

#[test]
fn vreduceps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM20)), operand3: Some(Literal8(97)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 51, 125, 139, 86, 244, 97], OperandSize::Qword)
}

#[test]
fn vreduceps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(XMM28)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 1213884811, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(56)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 99, 125, 140, 86, 36, 253, 139, 105, 90, 72, 56], OperandSize::Qword)
}

#[test]
fn vreduceps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(XMM25)), operand2: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 99, 125, 155, 86, 11, 0], OperandSize::Qword)
}

#[test]
fn vreduceps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Literal8(119)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 125, 174, 86, 224, 119], OperandSize::Dword)
}

#[test]
fn vreduceps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 1205763286, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 125, 174, 86, 36, 133, 214, 124, 222, 71, 0], OperandSize::Dword)
}

#[test]
fn vreduceps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(YMM7)), operand2: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand3: Some(Literal8(81)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 125, 185, 86, 63, 81], OperandSize::Dword)
}

#[test]
fn vreduceps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM19)), operand3: Some(Literal8(14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 163, 125, 174, 86, 219, 14], OperandSize::Qword)
}

#[test]
fn vreduceps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(YMM23)), operand2: Some(IndirectDisplaced(RSI, 63148884, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(75)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 227, 125, 171, 86, 190, 84, 147, 195, 3, 75], OperandSize::Qword)
}

#[test]
fn vreduceps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(YMM20)), operand2: Some(IndirectScaledIndexed(RDI, RBX, Eight, Some(OperandSize::Dword), None)), operand3: Some(Literal8(97)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 227, 125, 189, 86, 36, 223, 97], OperandSize::Qword)
}

#[test]
fn vreduceps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM6)), operand3: Some(Literal8(91)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 125, 159, 86, 254, 91], OperandSize::Dword)
}

#[test]
fn vreduceps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 264620451, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(67)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 125, 205, 86, 44, 149, 163, 201, 197, 15, 67], OperandSize::Dword)
}

#[test]
fn vreduceps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(ZMM6)), operand2: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand3: Some(Literal8(92)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 125, 218, 86, 55, 92], OperandSize::Dword)
}

#[test]
fn vreduceps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM8)), operand3: Some(Literal8(57)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 211, 125, 156, 86, 240, 57], OperandSize::Qword)
}

#[test]
fn vreduceps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(ZMM3)), operand2: Some(Indirect(RCX, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 125, 205, 86, 25, 18], OperandSize::Qword)
}

#[test]
fn vreduceps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledDisplaced(RSI, Four, 553298572, Some(OperandSize::Dword), None)), operand3: Some(Literal8(52)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 125, 219, 86, 12, 181, 140, 170, 250, 32, 52], OperandSize::Qword)
}

