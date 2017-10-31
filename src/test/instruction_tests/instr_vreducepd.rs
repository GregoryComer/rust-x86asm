use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vreducepd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(47)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 253, 137, 86, 246, 47], OperandSize::Dword)
}

#[test]
fn vreducepd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Two, 1310833526, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 253, 142, 86, 180, 70, 118, 187, 33, 78, 10], OperandSize::Dword)
}

#[test]
fn vreducepd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Four, 863560257, Some(OperandSize::Qword), None)), operand3: Some(Literal8(29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 253, 155, 86, 164, 191, 65, 226, 120, 51, 29], OperandSize::Dword)
}

#[test]
fn vreducepd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM23)), operand3: Some(Literal8(46)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 35, 253, 142, 86, 231, 46], OperandSize::Qword)
}

#[test]
fn vreducepd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(XMM24)), operand2: Some(IndirectScaledIndexed(RAX, RBX, Two, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(106)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 99, 253, 138, 86, 4, 88, 106], OperandSize::Qword)
}

#[test]
fn vreducepd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(XMM27)), operand2: Some(IndirectDisplaced(RCX, 1260622505, Some(OperandSize::Qword), None)), operand3: Some(Literal8(61)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 99, 253, 158, 86, 153, 169, 146, 35, 75, 61], OperandSize::Qword)
}

#[test]
fn vreducepd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(Literal8(36)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 253, 172, 86, 196, 36], OperandSize::Dword)
}

#[test]
fn vreducepd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(YMM0)), operand2: Some(IndirectDisplaced(EDI, 131477691, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(88)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 253, 172, 86, 135, 187, 48, 214, 7, 88], OperandSize::Dword)
}

#[test]
fn vreducepd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexed(EBX, EBX, Two, Some(OperandSize::Qword), None)), operand3: Some(Literal8(83)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 253, 191, 86, 36, 91, 83], OperandSize::Dword)
}

#[test]
fn vreducepd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM18)), operand3: Some(Literal8(64)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 35, 253, 173, 86, 194, 64], OperandSize::Qword)
}

#[test]
fn vreducepd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(YMM18)), operand2: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 227, 253, 171, 86, 19, 1], OperandSize::Qword)
}

#[test]
fn vreducepd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(YMM28)), operand2: Some(IndirectDisplaced(RBX, 1648851599, Some(OperandSize::Qword), None)), operand3: Some(Literal8(86)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 99, 253, 185, 86, 163, 143, 122, 71, 98, 86], OperandSize::Qword)
}

#[test]
fn vreducepd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(Literal8(103)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 253, 156, 86, 246, 103], OperandSize::Dword)
}

#[test]
fn vreducepd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(ZMM2)), operand2: Some(Indirect(EDI, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(77)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 253, 205, 86, 23, 77], OperandSize::Dword)
}

#[test]
fn vreducepd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledDisplaced(ECX, Four, 1448240391, Some(OperandSize::Qword), None)), operand3: Some(Literal8(92)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 253, 219, 86, 4, 141, 7, 101, 82, 86, 92], OperandSize::Dword)
}

#[test]
fn vreducepd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM27)), operand3: Some(Literal8(91)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 147, 253, 155, 86, 219, 91], OperandSize::Qword)
}

#[test]
fn vreducepd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(ZMM24)), operand2: Some(IndirectScaledIndexed(RSI, RAX, Eight, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 99, 253, 206, 86, 4, 198, 16], OperandSize::Qword)
}

#[test]
fn vreducepd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(ZMM13)), operand2: Some(IndirectDisplaced(RAX, 416916563, Some(OperandSize::Qword), None)), operand3: Some(Literal8(21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 115, 253, 218, 86, 168, 83, 164, 217, 24, 21], OperandSize::Qword)
}

