use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vshufi32x4_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFI32x4, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM7)), operand4: Some(Literal8(78)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 77, 202, 67, 199, 78], OperandSize::Dword)
}

#[test]
fn vshufi32x4_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFI32x4, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexed(ESI, EDI, Eight, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(27)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 77, 207, 67, 4, 254, 27], OperandSize::Dword)
}

#[test]
fn vshufi32x4_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFI32x4, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Two, 1119333125, Some(OperandSize::Dword), None)), operand4: Some(Literal8(102)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 85, 220, 67, 148, 90, 5, 171, 183, 66, 102], OperandSize::Dword)
}

#[test]
fn vshufi32x4_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFI32x4, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM14)), operand4: Some(Literal8(38)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 195, 85, 201, 67, 246, 38], OperandSize::Qword)
}

#[test]
fn vshufi32x4_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFI32x4, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM29)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Two, 1349914061, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(125)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 115, 21, 199, 67, 164, 118, 205, 13, 118, 80, 125], OperandSize::Qword)
}

#[test]
fn vshufi32x4_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFI32x4, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM18)), operand3: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand4: Some(Literal8(117)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 115, 109, 213, 67, 57, 117], OperandSize::Qword)
}

