use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpud_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM7)), operand4: Some(Literal8(24)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 93, 11, 30, 207, 24], OperandSize::Dword)
}

#[test]
fn vpcmpud_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(EAX, 68120713, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(29)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 101, 10, 30, 184, 137, 112, 15, 4, 29], OperandSize::Dword)
}

#[test]
fn vpcmpud_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(EDX, EDX, Four, Some(OperandSize::Dword), None)), operand4: Some(Literal8(31)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 77, 25, 30, 28, 146, 31], OperandSize::Dword)
}

#[test]
fn vpcmpud_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM28)), operand3: Some(Direct(XMM27)), operand4: Some(Literal8(48)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 147, 29, 2, 30, 243, 48], OperandSize::Qword)
}

#[test]
fn vpcmpud_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM27)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 1833116782, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(78)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 37, 7, 30, 44, 141, 110, 36, 67, 109, 78], OperandSize::Qword)
}

#[test]
fn vpcmpud_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Eight, 759270060, Some(OperandSize::Dword), None)), operand4: Some(Literal8(119)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 117, 29, 30, 140, 199, 172, 138, 65, 45, 119], OperandSize::Qword)
}

#[test]
fn vpcmpud_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM3)), operand4: Some(Literal8(74)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 93, 41, 30, 227, 74], OperandSize::Dword)
}

#[test]
fn vpcmpud_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(ESI, ESI, Eight, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(90)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 125, 41, 30, 60, 246, 90], OperandSize::Dword)
}

#[test]
fn vpcmpud_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(EAX, EDX, Two, Some(OperandSize::Dword), None)), operand4: Some(Literal8(29)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 101, 63, 30, 20, 80, 29], OperandSize::Dword)
}

#[test]
fn vpcmpud_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM10)), operand3: Some(Direct(YMM21)), operand4: Some(Literal8(91)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 179, 45, 43, 30, 245, 91], OperandSize::Qword)
}

#[test]
fn vpcmpud_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(89)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 109, 43, 30, 48, 89], OperandSize::Qword)
}

#[test]
fn vpcmpud_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM9)), operand3: Some(IndirectDisplaced(RDI, 1144123915, Some(OperandSize::Dword), None)), operand4: Some(Literal8(98)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 53, 62, 30, 159, 11, 242, 49, 68, 98], OperandSize::Qword)
}

#[test]
fn vpcmpud_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM3)), operand4: Some(Literal8(72)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 77, 79, 30, 219, 72], OperandSize::Dword)
}

#[test]
fn vpcmpud_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectDisplaced(EDX, 1582132256, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(26)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 77, 75, 30, 170, 32, 108, 77, 94, 26], OperandSize::Dword)
}

#[test]
fn vpcmpud_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM2)), operand3: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand4: Some(Literal8(45)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 109, 94, 30, 11, 45], OperandSize::Dword)
}

#[test]
fn vpcmpud_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM11)), operand3: Some(Direct(ZMM0)), operand4: Some(Literal8(119)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 37, 79, 30, 232, 119], OperandSize::Qword)
}

#[test]
fn vpcmpud_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM12)), operand3: Some(IndirectScaledIndexed(RBX, RAX, Two, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(85)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 29, 74, 30, 44, 67, 85], OperandSize::Qword)
}

#[test]
fn vpcmpud_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM26)), operand3: Some(IndirectDisplaced(RBX, 818441152, Some(OperandSize::Dword), None)), operand4: Some(Literal8(55)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 45, 85, 30, 155, 192, 107, 200, 48, 55], OperandSize::Qword)
}

