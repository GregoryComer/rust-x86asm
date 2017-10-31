use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vshufi64x2_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFI64x2, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM6)), operand4: Some(Literal8(126)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 245, 202, 67, 238, 126], OperandSize::Dword)
}

#[test]
fn vshufi64x2_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFI64x2, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectDisplaced(ESI, 946465314, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(104)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 197, 202, 67, 150, 34, 234, 105, 56, 104], OperandSize::Dword)
}

#[test]
fn vshufi64x2_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFI64x2, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM1)), operand3: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand4: Some(Literal8(2)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 245, 218, 67, 2, 2], OperandSize::Dword)
}

#[test]
fn vshufi64x2_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFI64x2, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM26)), operand4: Some(Literal8(72)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 19, 253, 205, 67, 234, 72], OperandSize::Qword)
}

#[test]
fn vshufi64x2_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFI64x2, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM23)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Eight, 2008981427, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(113)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 99, 197, 195, 67, 188, 251, 179, 159, 190, 119, 113], OperandSize::Qword)
}

#[test]
fn vshufi64x2_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFI64x2, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM18)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 1741743287, Some(OperandSize::Qword), None)), operand4: Some(Literal8(10)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 227, 237, 214, 67, 36, 253, 183, 228, 208, 103, 10], OperandSize::Qword)
}

