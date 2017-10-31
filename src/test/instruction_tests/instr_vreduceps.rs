use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vreduceps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(79)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 125, 143, 86, 234, 79], OperandSize::Dword)
}

#[test]
fn vreduceps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Eight, 210445986, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 125, 142, 86, 148, 207, 162, 38, 139, 12, 24], OperandSize::Dword)
}

#[test]
fn vreduceps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(EDX, 2011288341, Some(OperandSize::Dword), None)), operand3: Some(Literal8(53)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 125, 156, 86, 154, 21, 211, 225, 119, 53], OperandSize::Dword)
}

#[test]
fn vreduceps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM19)), operand3: Some(Literal8(56)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 179, 125, 142, 86, 243, 56], OperandSize::Qword)
}

#[test]
fn vreduceps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(123)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 125, 141, 86, 16, 123], OperandSize::Qword)
}

#[test]
fn vreduceps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 486365842, Some(OperandSize::Dword), None)), operand3: Some(Literal8(106)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 125, 159, 86, 60, 149, 146, 90, 253, 28, 106], OperandSize::Qword)
}

#[test]
fn vreduceps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(92)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 125, 175, 86, 206, 92], OperandSize::Dword)
}

#[test]
fn vreduceps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(YMM1)), operand2: Some(IndirectDisplaced(EDI, 1144524621, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(116)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 125, 169, 86, 143, 77, 15, 56, 68, 116], OperandSize::Dword)
}

#[test]
fn vreduceps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(YMM3)), operand2: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand3: Some(Literal8(51)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 125, 191, 86, 31, 51], OperandSize::Dword)
}

#[test]
fn vreduceps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM13)), operand3: Some(Literal8(3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 195, 125, 170, 86, 229, 3], OperandSize::Qword)
}

#[test]
fn vreduceps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(YMM23)), operand2: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 227, 125, 169, 86, 58, 7], OperandSize::Qword)
}

#[test]
fn vreduceps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(YMM16)), operand2: Some(IndirectScaledIndexed(RBX, RDI, Eight, Some(OperandSize::Dword), None)), operand3: Some(Literal8(105)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 227, 125, 185, 86, 4, 251, 105], OperandSize::Qword)
}

#[test]
fn vreduceps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM0)), operand3: Some(Literal8(75)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 125, 156, 86, 200, 75], OperandSize::Dword)
}

#[test]
fn vreduceps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledIndexed(EDI, EBX, Four, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(118)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 125, 205, 86, 12, 159, 118], OperandSize::Dword)
}

#[test]
fn vreduceps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 968511809, Some(OperandSize::Dword), None)), operand3: Some(Literal8(50)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 125, 223, 86, 12, 117, 65, 81, 186, 57, 50], OperandSize::Dword)
}

#[test]
fn vreduceps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM25)), operand3: Some(Literal8(76)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 19, 125, 153, 86, 241, 76], OperandSize::Qword)
}

#[test]
fn vreduceps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Eight, 829012928, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(37)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 125, 207, 86, 148, 223, 192, 187, 105, 49, 37], OperandSize::Qword)
}

#[test]
fn vreduceps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(ZMM16)), operand2: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand3: Some(Literal8(53)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 227, 125, 223, 86, 6, 53], OperandSize::Qword)
}

