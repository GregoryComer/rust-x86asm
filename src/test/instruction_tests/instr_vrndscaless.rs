use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrndscaless_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALESS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(40)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 69, 153, 10, 244, 40], OperandSize::Dword)
}

#[test]
fn vrndscaless_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALESS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EAX, 1841222757, Some(OperandSize::Dword), None)), operand4: Some(Literal8(109)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 77, 140, 10, 136, 101, 212, 190, 109, 109], OperandSize::Dword)
}

#[test]
fn vrndscaless_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALESS, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM23)), operand3: Some(Direct(XMM8)), operand4: Some(Literal8(3)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 195, 69, 149, 10, 224, 3], OperandSize::Qword)
}

#[test]
fn vrndscaless_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALESS, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM12)), operand3: Some(IndirectDisplaced(RSI, 1182344963, Some(OperandSize::Dword), None)), operand4: Some(Literal8(113)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 227, 29, 138, 10, 190, 3, 39, 121, 70, 113], OperandSize::Qword)
}

