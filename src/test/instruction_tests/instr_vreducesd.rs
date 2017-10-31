use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vreducesd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCESD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: Some(Literal8(71)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 245, 159, 87, 214, 71], OperandSize::Dword)
}

#[test]
fn vreducesd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCESD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 304410097, Some(OperandSize::Qword), None)), operand4: Some(Literal8(41)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 205, 141, 87, 20, 213, 241, 237, 36, 18, 41], OperandSize::Dword)
}

#[test]
fn vreducesd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCESD, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM18)), operand3: Some(Direct(XMM28)), operand4: Some(Literal8(53)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 19, 237, 145, 87, 204, 53], OperandSize::Qword)
}

#[test]
fn vreducesd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCESD, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM27)), operand3: Some(IndirectDisplaced(RBX, 97222277, Some(OperandSize::Qword), None)), operand4: Some(Literal8(28)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 227, 165, 133, 87, 147, 133, 126, 203, 5, 28], OperandSize::Qword)
}

