use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrangesd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGESD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(98)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 229, 154, 81, 212, 98], OperandSize::Dword)
}

#[test]
fn vrangesd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGESD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(EAX, EAX, Two, Some(OperandSize::Qword), None)), operand4: Some(Literal8(74)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 197, 139, 81, 36, 64, 74], OperandSize::Dword)
}

#[test]
fn vrangesd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGESD, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM8)), operand3: Some(Direct(XMM15)), operand4: Some(Literal8(95)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 67, 189, 154, 81, 247, 95], OperandSize::Qword)
}

#[test]
fn vrangesd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGESD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM24)), operand3: Some(IndirectDisplaced(RSI, 1246247450, Some(OperandSize::Qword), None)), operand4: Some(Literal8(109)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 189, 135, 81, 134, 26, 58, 72, 74, 109], OperandSize::Qword)
}

