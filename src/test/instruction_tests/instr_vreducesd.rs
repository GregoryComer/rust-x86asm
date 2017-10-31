use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vreducesd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCESD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: Some(Literal8(43)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 237, 153, 87, 217, 43], OperandSize::Dword)
}

#[test]
fn vreducesd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCESD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Four, 1940720713, Some(OperandSize::Qword), None)), operand4: Some(Literal8(6)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 221, 143, 87, 156, 183, 73, 12, 173, 115, 6], OperandSize::Dword)
}

#[test]
fn vreducesd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCESD, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM19)), operand3: Some(Direct(XMM14)), operand4: Some(Literal8(87)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 195, 229, 145, 87, 246, 87], OperandSize::Qword)
}

#[test]
fn vreducesd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCESD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Eight, 873004931, Some(OperandSize::Qword), None)), operand4: Some(Literal8(87)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 205, 143, 87, 140, 223, 131, 255, 8, 52, 87], OperandSize::Qword)
}

