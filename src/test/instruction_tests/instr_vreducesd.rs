use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vreducesd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCESD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(99)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 197, 153, 87, 234, 99], OperandSize::Dword)
}

#[test]
fn vreducesd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCESD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(EAX, 605194400, Some(OperandSize::Qword), None)), operand4: Some(Literal8(100)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 245, 142, 87, 168, 160, 136, 18, 36, 100], OperandSize::Dword)
}

#[test]
fn vreducesd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCESD, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM30)), operand3: Some(Direct(XMM22)), operand4: Some(Literal8(98)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 163, 141, 149, 87, 206, 98], OperandSize::Qword)
}

#[test]
fn vreducesd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCESD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 34866823, Some(OperandSize::Qword), None)), operand4: Some(Literal8(36)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 221, 139, 87, 52, 189, 135, 6, 20, 2, 36], OperandSize::Qword)
}

