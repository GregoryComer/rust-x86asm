use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vshuff32x4_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFF32x4, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM6)), operand4: Some(Literal8(118)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 69, 205, 35, 222, 118], OperandSize::Dword)
}

#[test]
fn vshuff32x4_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFF32x4, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexed(ECX, EBX, Two, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(98)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 101, 205, 35, 52, 89, 98], OperandSize::Dword)
}

#[test]
fn vshuff32x4_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFF32x4, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexed(EAX, EAX, Eight, Some(OperandSize::Dword), None)), operand4: Some(Literal8(106)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 69, 222, 35, 20, 192, 106], OperandSize::Dword)
}

#[test]
fn vshuff32x4_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFF32x4, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM24)), operand3: Some(Direct(ZMM10)), operand4: Some(Literal8(91)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 83, 61, 196, 35, 202, 91], OperandSize::Qword)
}

#[test]
fn vshuff32x4_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFF32x4, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectDisplaced(RSI, 280746854, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(15)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 115, 93, 203, 35, 158, 102, 219, 187, 16, 15], OperandSize::Qword)
}

#[test]
fn vshuff32x4_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFF32x4, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM18)), operand3: Some(IndirectScaledIndexed(RBX, RBX, Four, Some(OperandSize::Dword), None)), operand4: Some(Literal8(33)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 99, 109, 213, 35, 20, 155, 33], OperandSize::Qword)
}

