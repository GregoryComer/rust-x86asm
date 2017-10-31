use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrsqrt28sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28SD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 205, 157, 205, 251], OperandSize::Dword)
}

#[test]
fn vrsqrt28sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 141, 205, 38], OperandSize::Dword)
}

#[test]
fn vrsqrt28sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28SD, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM14)), operand3: Some(Direct(XMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 34, 141, 157, 205, 218], OperandSize::Qword)
}

#[test]
fn vrsqrt28sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 163341686, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 213, 139, 205, 12, 85, 118, 101, 188, 9], OperandSize::Qword)
}

