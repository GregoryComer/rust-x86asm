use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrangesd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGESD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: Some(Literal8(127)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 213, 154, 81, 232, 127], OperandSize::Dword)
}

#[test]
fn vrangesd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGESD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EAX, 838581409, Some(OperandSize::Qword), None)), operand4: Some(Literal8(114)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 205, 139, 81, 144, 161, 188, 251, 49, 114], OperandSize::Dword)
}

#[test]
fn vrangesd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGESD, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM11)), operand3: Some(Direct(XMM5)), operand4: Some(Literal8(20)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 227, 165, 155, 81, 237, 20], OperandSize::Qword)
}

#[test]
fn vrangesd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGESD, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM10)), operand3: Some(IndirectScaledIndexed(RSI, RDI, Eight, Some(OperandSize::Qword), None)), operand4: Some(Literal8(55)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 115, 173, 140, 81, 60, 254, 55], OperandSize::Qword)
}

