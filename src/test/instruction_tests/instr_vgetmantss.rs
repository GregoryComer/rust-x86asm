use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vgetmantss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(124)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 125, 157, 39, 242, 124], OperandSize::Dword)
}

#[test]
fn vgetmantss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 253895463, Some(OperandSize::Dword), None)), operand4: Some(Literal8(19)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 77, 140, 39, 44, 221, 39, 35, 34, 15, 19], OperandSize::Dword)
}

#[test]
fn vgetmantss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM21)), operand3: Some(Direct(XMM31)), operand4: Some(Literal8(51)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 147, 85, 148, 39, 215, 51], OperandSize::Qword)
}

#[test]
fn vgetmantss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTSS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM30)), operand3: Some(IndirectDisplaced(RBX, 400230097, Some(OperandSize::Dword), None)), operand4: Some(Literal8(62)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 13, 132, 39, 155, 209, 6, 219, 23, 62], OperandSize::Qword)
}

