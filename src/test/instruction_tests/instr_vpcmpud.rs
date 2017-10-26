use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpud_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(68)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 77, 13, 30, 210, 68], OperandSize::Dword)
}

#[test]
fn vpcmpud_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(EBX, 1247446587, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(17)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 109, 9, 30, 171, 59, 134, 90, 74, 17], OperandSize::Dword)
}

#[test]
fn vpcmpud_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(EBX, EDI, Eight, Some(OperandSize::Dword), None)), operand4: Some(Literal8(70)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 125, 27, 30, 28, 251, 70], OperandSize::Dword)
}

#[test]
fn vpcmpud_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: Some(Literal8(120)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 117, 10, 30, 243, 120], OperandSize::Qword)
}

#[test]
fn vpcmpud_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM24)), operand3: Some(IndirectScaledIndexed(RDX, RAX, Four, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(3)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 61, 1, 30, 60, 130, 3], OperandSize::Qword)
}

#[test]
fn vpcmpud_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(RAX, RAX, Two, Some(OperandSize::Dword), None)), operand4: Some(Literal8(76)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 101, 27, 30, 60, 64, 76], OperandSize::Qword)
}

#[test]
fn vpcmpud_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM5)), operand4: Some(Literal8(114)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 93, 44, 30, 245, 114], OperandSize::Dword)
}

#[test]
fn vpcmpud_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(103)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 109, 47, 30, 55, 103], OperandSize::Dword)
}

#[test]
fn vpcmpud_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(EDI, ESI, Four, Some(OperandSize::Dword), None)), operand4: Some(Literal8(71)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 109, 57, 30, 20, 183, 71], OperandSize::Dword)
}

#[test]
fn vpcmpud_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM16)), operand3: Some(Direct(YMM3)), operand4: Some(Literal8(5)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 125, 37, 30, 227, 5], OperandSize::Qword)
}

#[test]
fn vpcmpud_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM18)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Two, 1635418840, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(51)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 109, 35, 30, 164, 81, 216, 130, 122, 97, 51], OperandSize::Qword)
}

#[test]
fn vpcmpud_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(RDI, RDX, Two, Some(OperandSize::Dword), None)), operand4: Some(Literal8(99)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 77, 62, 30, 52, 87, 99], OperandSize::Qword)
}

#[test]
fn vpcmpud_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM2)), operand4: Some(Literal8(20)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 117, 78, 30, 234, 20], OperandSize::Dword)
}

#[test]
fn vpcmpud_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM4)), operand3: Some(Indirect(EBX, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(108)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 93, 79, 30, 11, 108], OperandSize::Dword)
}

#[test]
fn vpcmpud_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectDisplaced(EDX, 1302236797, Some(OperandSize::Dword), None)), operand4: Some(Literal8(51)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 109, 93, 30, 178, 125, 142, 158, 77, 51], OperandSize::Dword)
}

#[test]
fn vpcmpud_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM21)), operand4: Some(Literal8(118)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 179, 125, 73, 30, 205, 118], OperandSize::Qword)
}

#[test]
fn vpcmpud_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM28)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Two, 1025691736, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(123)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 29, 65, 30, 180, 82, 88, 208, 34, 61, 123], OperandSize::Qword)
}

#[test]
fn vpcmpud_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM13)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Two, 28114205, Some(OperandSize::Dword), None)), operand4: Some(Literal8(67)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 21, 95, 30, 172, 121, 29, 253, 172, 1, 67], OperandSize::Qword)
}

