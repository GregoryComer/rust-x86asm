use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vgetmantss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM7)), operand4: Some(Literal8(113)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 109, 154, 39, 231, 113], OperandSize::Dword)
}

#[test]
fn vgetmantss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand4: Some(Literal8(60)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 125, 139, 39, 51, 60], OperandSize::Dword)
}

#[test]
fn vgetmantss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTSS, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM11)), operand4: Some(Literal8(11)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 195, 109, 156, 39, 219, 11], OperandSize::Qword)
}

#[test]
fn vgetmantss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTSS, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(RDX, RAX, Two, Some(OperandSize::Dword), None)), operand4: Some(Literal8(81)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 227, 101, 138, 39, 20, 66, 81], OperandSize::Qword)
}

