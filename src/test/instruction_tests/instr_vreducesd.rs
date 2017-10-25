use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vreducesd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCESD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(7)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 229, 159, 87, 196, 7], OperandSize::Dword)
}

#[test]
fn vreducesd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCESD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand4: Some(Literal8(3)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 237, 137, 87, 46, 3], OperandSize::Dword)
}

#[test]
fn vreducesd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCESD, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM15)), operand4: Some(Literal8(104)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 83, 213, 156, 87, 231, 104], OperandSize::Qword)
}

#[test]
fn vreducesd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCESD, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM20)), operand3: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand4: Some(Literal8(116)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 99, 221, 129, 87, 27, 116], OperandSize::Qword)
}

