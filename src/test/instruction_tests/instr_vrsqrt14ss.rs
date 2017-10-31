use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrsqrt14ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14SS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 85, 138, 79, 212], OperandSize::Dword)
}

#[test]
fn vrsqrt14ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 79, 41], OperandSize::Dword)
}

#[test]
fn vrsqrt14ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14SS, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM21)), operand3: Some(Direct(XMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 162, 85, 134, 79, 251], OperandSize::Qword)
}

#[test]
fn vrsqrt14ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14SS, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 2030979021, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 101, 142, 79, 20, 85, 205, 71, 14, 121], OperandSize::Qword)
}

