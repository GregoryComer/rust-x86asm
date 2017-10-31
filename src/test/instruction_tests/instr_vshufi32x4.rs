use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vshufi32x4_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFI32x4, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM6)), operand4: Some(Literal8(56)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 93, 205, 67, 222, 56], OperandSize::Dword)
}

#[test]
fn vshufi32x4_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFI32x4, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexed(EBX, EDX, Four, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(99)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 77, 203, 67, 44, 147, 99], OperandSize::Dword)
}

#[test]
fn vshufi32x4_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFI32x4, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Four, 310070582, Some(OperandSize::Dword), None)), operand4: Some(Literal8(94)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 117, 219, 67, 132, 154, 54, 77, 123, 18, 94], OperandSize::Dword)
}

#[test]
fn vshufi32x4_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFI32x4, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM25)), operand3: Some(Direct(ZMM14)), operand4: Some(Literal8(52)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 67, 53, 198, 67, 206, 52], OperandSize::Qword)
}

#[test]
fn vshufi32x4_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFI32x4, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM24)), operand3: Some(IndirectScaledIndexed(RCX, RAX, Eight, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(118)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 227, 61, 194, 67, 60, 193, 118], OperandSize::Qword)
}

#[test]
fn vshufi32x4_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFI32x4, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Two, 799223817, Some(OperandSize::Dword), None)), operand4: Some(Literal8(56)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 115, 109, 219, 67, 180, 75, 9, 48, 163, 47, 56], OperandSize::Qword)
}

