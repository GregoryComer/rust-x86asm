use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vshuff32x4_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFF32x4, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM5)), operand4: Some(Literal8(92)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 117, 202, 35, 213, 92], OperandSize::Dword)
}

#[test]
fn vshuff32x4_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFF32x4, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectDisplaced(EAX, 423217455, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(118)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 69, 205, 35, 160, 47, 201, 57, 25, 118], OperandSize::Dword)
}

#[test]
fn vshuff32x4_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFF32x4, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectDisplaced(EDI, 2134355939, Some(OperandSize::Dword), None)), operand4: Some(Literal8(118)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 77, 221, 35, 183, 227, 175, 55, 127, 118], OperandSize::Dword)
}

#[test]
fn vshuff32x4_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFF32x4, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM20)), operand3: Some(Direct(ZMM22)), operand4: Some(Literal8(51)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 163, 93, 194, 35, 222, 51], OperandSize::Qword)
}

#[test]
fn vshuff32x4_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFF32x4, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Eight, 1206524353, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(122)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 99, 77, 204, 35, 148, 251, 193, 25, 234, 71, 122], OperandSize::Qword)
}

#[test]
fn vshuff32x4_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFF32x4, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 689740214, Some(OperandSize::Dword), None)), operand4: Some(Literal8(40)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 115, 93, 221, 35, 52, 125, 182, 153, 28, 41, 40], OperandSize::Qword)
}

