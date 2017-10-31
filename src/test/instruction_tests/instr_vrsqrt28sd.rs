use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrsqrt28sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 245, 156, 205, 235], OperandSize::Dword)
}

#[test]
fn vrsqrt28sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Four, 73917731, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 213, 142, 205, 180, 144, 35, 229, 103, 4], OperandSize::Dword)
}

#[test]
fn vrsqrt28sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28SD, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM18)), operand3: Some(Direct(XMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 194, 237, 151, 205, 226], OperandSize::Qword)
}

#[test]
fn vrsqrt28sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28SD, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM11)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 1225801387, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 165, 137, 205, 4, 77, 171, 62, 16, 73], OperandSize::Qword)
}

