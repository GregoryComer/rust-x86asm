use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrndscaless_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALESS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM6)), operand4: Some(Literal8(88)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 69, 155, 10, 230, 88], OperandSize::Dword)
}

#[test]
fn vrndscaless_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALESS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Two, 1707027016, Some(OperandSize::Dword), None)), operand4: Some(Literal8(69)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 85, 138, 10, 148, 66, 72, 42, 191, 101, 69], OperandSize::Dword)
}

#[test]
fn vrndscaless_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALESS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM21)), operand4: Some(Literal8(91)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 179, 109, 158, 10, 213, 91], OperandSize::Qword)
}

#[test]
fn vrndscaless_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALESS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM10)), operand3: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand4: Some(Literal8(69)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 45, 143, 10, 40, 69], OperandSize::Qword)
}

