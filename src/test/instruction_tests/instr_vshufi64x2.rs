use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vshufi64x2_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFI64x2, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM4)), operand4: Some(Literal8(75)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 205, 203, 67, 212, 75], OperandSize::Dword)
}

#[test]
fn vshufi64x2_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFI64x2, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Two, 361359170, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(105)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 213, 203, 67, 188, 120, 66, 231, 137, 21, 105], OperandSize::Dword)
}

#[test]
fn vshufi64x2_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFI64x2, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectDisplaced(EBX, 1760745323, Some(OperandSize::Qword), None)), operand4: Some(Literal8(32)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 253, 223, 67, 155, 107, 215, 242, 104, 32], OperandSize::Dword)
}

#[test]
fn vshufi64x2_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFI64x2, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM21)), operand3: Some(Direct(ZMM14)), operand4: Some(Literal8(113)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 211, 213, 195, 67, 238, 113], OperandSize::Qword)
}

#[test]
fn vshufi64x2_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFI64x2, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM14)), operand3: Some(Indirect(RBX, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(70)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 115, 141, 206, 67, 43, 70], OperandSize::Qword)
}

#[test]
fn vshufi64x2_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFI64x2, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM22)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Eight, 1776154366, Some(OperandSize::Qword), None)), operand4: Some(Literal8(90)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 115, 205, 212, 67, 164, 240, 254, 246, 221, 105, 90], OperandSize::Qword)
}

