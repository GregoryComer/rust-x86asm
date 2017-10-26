use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vgetmantss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: Some(Literal8(74)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 101, 157, 39, 193, 74], OperandSize::Dword)
}

#[test]
fn vgetmantss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 1365966431, Some(OperandSize::Dword), None)), operand4: Some(Literal8(125)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 117, 143, 39, 20, 197, 95, 254, 106, 81, 125], OperandSize::Dword)
}

#[test]
fn vgetmantss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTSS, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM7)), operand4: Some(Literal8(41)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 99, 109, 154, 39, 231, 41], OperandSize::Qword)
}

#[test]
fn vgetmantss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTSS, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM14)), operand3: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand4: Some(Literal8(96)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 227, 13, 139, 39, 55, 96], OperandSize::Qword)
}

