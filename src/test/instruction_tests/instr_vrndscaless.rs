use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrndscaless_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALESS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: Some(Literal8(58)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 109, 156, 10, 238, 58], OperandSize::Dword)
}

#[test]
fn vrndscaless_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALESS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 1467466498, Some(OperandSize::Dword), None)), operand4: Some(Literal8(21)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 101, 141, 10, 60, 213, 2, 195, 119, 87, 21], OperandSize::Dword)
}

#[test]
fn vrndscaless_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALESS, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(18)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 227, 125, 156, 10, 196, 18], OperandSize::Qword)
}

#[test]
fn vrndscaless_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALESS, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM24)), operand3: Some(IndirectDisplaced(RAX, 81129585, Some(OperandSize::Dword), None)), operand4: Some(Literal8(47)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 227, 61, 132, 10, 136, 113, 240, 213, 4, 47], OperandSize::Qword)
}

