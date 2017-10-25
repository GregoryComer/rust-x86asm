use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrsqrt28ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 93, 158, 205, 254], OperandSize::Dword)
}

#[test]
fn vrsqrt28ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 2000855261, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 137, 205, 4, 85, 221, 160, 66, 119], OperandSize::Dword)
}

#[test]
fn vrsqrt28ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28SS, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM26)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 45, 151, 205, 213], OperandSize::Qword)
}

#[test]
fn vrsqrt28ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28SS, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM9)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 1591045971, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 53, 141, 205, 60, 77, 83, 111, 213, 94], OperandSize::Qword)
}

