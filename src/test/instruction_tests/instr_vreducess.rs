use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vreducess_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCESS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(118)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 125, 157, 87, 194, 118], OperandSize::Dword)
}

#[test]
fn vreducess_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCESS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(EAX, 295949850, Some(OperandSize::Dword), None)), operand4: Some(Literal8(98)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 109, 138, 87, 176, 26, 214, 163, 17, 98], OperandSize::Dword)
}

#[test]
fn vreducess_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCESS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM15)), operand3: Some(Direct(XMM31)), operand4: Some(Literal8(67)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 147, 5, 155, 87, 199, 67], OperandSize::Qword)
}

#[test]
fn vreducess_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCESS, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM8)), operand3: Some(IndirectDisplaced(RCX, 742537669, Some(OperandSize::Dword), None)), operand4: Some(Literal8(9)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 115, 61, 139, 87, 129, 197, 57, 66, 44, 9], OperandSize::Qword)
}

