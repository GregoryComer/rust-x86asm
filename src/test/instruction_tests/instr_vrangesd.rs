use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrangesd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGESD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM0)), operand4: Some(Literal8(1)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 221, 157, 81, 200, 1], OperandSize::Dword)
}

#[test]
fn vrangesd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGESD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(ESI, 1769015178, Some(OperandSize::Qword), None)), operand4: Some(Literal8(98)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 229, 138, 81, 182, 138, 7, 113, 105, 98], OperandSize::Dword)
}

#[test]
fn vrangesd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGESD, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM19)), operand4: Some(Literal8(29)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 35, 237, 154, 81, 243, 29], OperandSize::Qword)
}

#[test]
fn vrangesd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGESD, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM31)), operand3: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand4: Some(Literal8(88)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 115, 133, 133, 81, 30, 88], OperandSize::Qword)
}

