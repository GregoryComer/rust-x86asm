use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vreduceps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(79)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 125, 140, 86, 211, 79], OperandSize::Dword)
}

#[test]
fn vreduceps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(EDX, 1522105546, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(87)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 125, 140, 86, 170, 202, 124, 185, 90, 87], OperandSize::Dword)
}

#[test]
fn vreduceps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(ECX, 1435911584, Some(OperandSize::Dword), None)), operand3: Some(Literal8(110)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 125, 158, 86, 161, 160, 69, 150, 85, 110], OperandSize::Dword)
}

#[test]
fn vreduceps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM26)), operand3: Some(Literal8(10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 3, 125, 141, 86, 250, 10], OperandSize::Qword)
}

#[test]
fn vreduceps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(RBX, Eight, 98519079, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(107)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 125, 140, 86, 20, 221, 39, 72, 223, 5, 107], OperandSize::Qword)
}

#[test]
fn vreduceps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Four, 1538560935, Some(OperandSize::Dword), None)), operand3: Some(Literal8(62)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 125, 153, 86, 180, 182, 167, 147, 180, 91, 62], OperandSize::Qword)
}

#[test]
fn vreduceps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 125, 174, 86, 229, 29], OperandSize::Dword)
}

#[test]
fn vreduceps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(YMM3)), operand2: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(111)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 125, 175, 86, 26, 111], OperandSize::Dword)
}

#[test]
fn vreduceps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(YMM5)), operand2: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand3: Some(Literal8(5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 125, 186, 86, 47, 5], OperandSize::Dword)
}

#[test]
fn vreduceps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM19)), operand3: Some(Literal8(29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 163, 125, 173, 86, 235, 29], OperandSize::Qword)
}

#[test]
fn vreduceps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(YMM30)), operand2: Some(IndirectScaledIndexed(RDX, RBX, Two, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(99)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 99, 125, 174, 86, 52, 90, 99], OperandSize::Qword)
}

#[test]
fn vreduceps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(YMM6)), operand2: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand3: Some(Literal8(69)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 125, 191, 86, 55, 69], OperandSize::Qword)
}

#[test]
fn vreduceps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM3)), operand3: Some(Literal8(12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 125, 153, 86, 203, 12], OperandSize::Dword)
}

#[test]
fn vreduceps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(ZMM1)), operand2: Some(Indirect(EDI, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(47)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 125, 206, 86, 15, 47], OperandSize::Dword)
}

#[test]
fn vreduceps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledIndexed(EDI, EAX, Eight, Some(OperandSize::Dword), None)), operand3: Some(Literal8(123)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 125, 222, 86, 4, 199, 123], OperandSize::Dword)
}

#[test]
fn vreduceps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM21)), operand3: Some(Literal8(123)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 51, 125, 153, 86, 237, 123], OperandSize::Qword)
}

#[test]
fn vreduceps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(ZMM13)), operand2: Some(IndirectScaledIndexed(RDI, RBX, Two, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(124)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 115, 125, 207, 86, 44, 95, 124], OperandSize::Qword)
}

#[test]
fn vreduceps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(ZMM15)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Two, 263348305, Some(OperandSize::Dword), None)), operand3: Some(Literal8(57)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 115, 125, 222, 86, 188, 86, 81, 96, 178, 15, 57], OperandSize::Qword)
}

