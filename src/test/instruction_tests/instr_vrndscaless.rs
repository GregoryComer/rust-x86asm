use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrndscaless_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALESS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: Some(Literal8(3)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 109, 155, 10, 214, 3], OperandSize::Dword)
}

#[test]
fn vrndscaless_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALESS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Four, 369510280, Some(OperandSize::Dword), None)), operand4: Some(Literal8(102)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 69, 143, 10, 180, 130, 136, 71, 6, 22, 102], OperandSize::Dword)
}

#[test]
fn vrndscaless_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALESS, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM25)), operand3: Some(Direct(XMM27)), operand4: Some(Literal8(32)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 19, 53, 146, 10, 243, 32], OperandSize::Qword)
}

#[test]
fn vrndscaless_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALESS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand4: Some(Literal8(122)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 93, 141, 10, 33, 122], OperandSize::Qword)
}

