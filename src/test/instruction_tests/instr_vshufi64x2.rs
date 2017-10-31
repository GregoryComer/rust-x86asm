use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vshufi64x2_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFI64x2, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM3)), operand4: Some(Literal8(118)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 197, 202, 67, 251, 118], OperandSize::Dword)
}

#[test]
fn vshufi64x2_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFI64x2, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Eight, 513441548, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(103)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 205, 202, 67, 148, 210, 12, 127, 154, 30, 103], OperandSize::Dword)
}

#[test]
fn vshufi64x2_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFI64x2, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexed(EAX, EBX, Eight, Some(OperandSize::Qword), None)), operand4: Some(Literal8(59)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 237, 222, 67, 4, 216, 59], OperandSize::Dword)
}

#[test]
fn vshufi64x2_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFI64x2, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM17)), operand3: Some(Direct(ZMM4)), operand4: Some(Literal8(27)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 115, 245, 193, 67, 204, 27], OperandSize::Qword)
}

#[test]
fn vshufi64x2_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFI64x2, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM8)), operand3: Some(IndirectDisplaced(RBX, 1705117041, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(72)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 99, 189, 204, 67, 163, 113, 5, 162, 101, 72], OperandSize::Qword)
}

#[test]
fn vshufi64x2_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFI64x2, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM16)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 444384596, Some(OperandSize::Qword), None)), operand4: Some(Literal8(70)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 115, 253, 214, 67, 20, 93, 84, 197, 124, 26, 70], OperandSize::Qword)
}

