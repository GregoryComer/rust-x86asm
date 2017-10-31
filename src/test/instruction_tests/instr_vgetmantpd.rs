use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vgetmantpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(46)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 253, 138, 38, 226, 46], OperandSize::Dword)
}

#[test]
fn vgetmantpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 405100471, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 253, 142, 38, 28, 117, 183, 87, 37, 24, 17], OperandSize::Dword)
}

#[test]
fn vgetmantpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand3: Some(Literal8(7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 253, 154, 38, 57, 7], OperandSize::Dword)
}

#[test]
fn vgetmantpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM31)), operand3: Some(Literal8(44)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 3, 253, 139, 38, 255, 44], OperandSize::Qword)
}

#[test]
fn vgetmantpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(XMM16)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 2139593831, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(69)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 227, 253, 140, 38, 4, 85, 103, 156, 135, 127, 69], OperandSize::Qword)
}

#[test]
fn vgetmantpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(XMM9)), operand2: Some(IndirectScaledDisplaced(RBX, Two, 1327931241, Some(OperandSize::Qword), None)), operand3: Some(Literal8(60)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 115, 253, 156, 38, 12, 93, 105, 159, 38, 79, 60], OperandSize::Qword)
}

#[test]
fn vgetmantpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Literal8(109)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 253, 172, 38, 217, 109], OperandSize::Dword)
}

#[test]
fn vgetmantpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Two, 482079588, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 253, 169, 38, 156, 86, 100, 243, 187, 28, 7], OperandSize::Dword)
}

#[test]
fn vgetmantpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(YMM3)), operand2: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand3: Some(Literal8(40)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 253, 187, 38, 30, 40], OperandSize::Dword)
}

#[test]
fn vgetmantpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM16)), operand3: Some(Literal8(65)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 163, 253, 172, 38, 232, 65], OperandSize::Qword)
}

#[test]
fn vgetmantpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(YMM13)), operand2: Some(IndirectDisplaced(RAX, 1766482510, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(70)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 115, 253, 170, 38, 168, 78, 98, 74, 105, 70], OperandSize::Qword)
}

#[test]
fn vgetmantpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(YMM15)), operand2: Some(IndirectDisplaced(RDX, 341897915, Some(OperandSize::Qword), None)), operand3: Some(Literal8(124)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 115, 253, 186, 38, 186, 187, 242, 96, 20, 124], OperandSize::Qword)
}

#[test]
fn vgetmantpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM2)), operand3: Some(Literal8(110)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 253, 155, 38, 210, 110], OperandSize::Dword)
}

#[test]
fn vgetmantpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(ZMM7)), operand2: Some(Indirect(ECX, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(112)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 253, 204, 38, 57, 112], OperandSize::Dword)
}

#[test]
fn vgetmantpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(ZMM3)), operand2: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand3: Some(Literal8(26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 253, 223, 38, 25, 26], OperandSize::Dword)
}

#[test]
fn vgetmantpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM12)), operand3: Some(Literal8(124)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 211, 253, 155, 38, 196, 124], OperandSize::Qword)
}

#[test]
fn vgetmantpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(ZMM29)), operand2: Some(IndirectScaledIndexed(RDX, RBX, Four, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(102)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 99, 253, 204, 38, 44, 154, 102], OperandSize::Qword)
}

#[test]
fn vgetmantpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(ZMM22)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Eight, 185199432, Some(OperandSize::Qword), None)), operand3: Some(Literal8(85)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 227, 253, 219, 38, 180, 240, 72, 235, 9, 11, 85], OperandSize::Qword)
}

