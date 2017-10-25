use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrsqrt28sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 205, 154, 205, 239], OperandSize::Dword)
}

#[test]
fn vrsqrt28sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(ECX, EDX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 143, 205, 20, 209], OperandSize::Dword)
}

#[test]
fn vrsqrt28sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28SD, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM11)), operand3: Some(Direct(XMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 82, 165, 156, 205, 253], OperandSize::Qword)
}

#[test]
fn vrsqrt28sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28SD, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM25)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Eight, 1670854710, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 181, 130, 205, 188, 223, 54, 56, 151, 99], OperandSize::Qword)
}

