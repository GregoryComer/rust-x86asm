use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vshufi64x2_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFI64x2, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM3)), operand4: Some(Literal8(83)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 237, 205, 67, 203, 83], OperandSize::Dword)
}

#[test]
fn vshufi64x2_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFI64x2, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 652522809, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(43)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 245, 204, 67, 4, 133, 57, 181, 228, 38, 43], OperandSize::Dword)
}

#[test]
fn vshufi64x2_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFI64x2, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 1448572356, Some(OperandSize::Qword), None)), operand4: Some(Literal8(31)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 229, 217, 67, 60, 69, 196, 117, 87, 86, 31], OperandSize::Dword)
}

#[test]
fn vshufi64x2_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFI64x2, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM8)), operand3: Some(Direct(ZMM20)), operand4: Some(Literal8(86)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 51, 189, 206, 67, 252, 86], OperandSize::Qword)
}

#[test]
fn vshufi64x2_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFI64x2, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM17)), operand3: Some(IndirectScaledIndexed(RDX, RBX, Four, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(38)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 115, 245, 193, 67, 52, 154, 38], OperandSize::Qword)
}

#[test]
fn vshufi64x2_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFI64x2, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM16)), operand3: Some(IndirectScaledIndexed(RDX, RCX, Four, Some(OperandSize::Qword), None)), operand4: Some(Literal8(109)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 253, 215, 67, 52, 138, 109], OperandSize::Qword)
}

