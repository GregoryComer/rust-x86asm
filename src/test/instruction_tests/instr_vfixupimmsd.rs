use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfixupimmsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(78)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 229, 157, 85, 252, 78], OperandSize::Dword)
}

#[test]
fn vfixupimmsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Two, 784174969, Some(OperandSize::Qword), None)), operand4: Some(Literal8(78)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 245, 137, 85, 140, 64, 121, 143, 189, 46, 78], OperandSize::Dword)
}

#[test]
fn vfixupimmsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM21)), operand3: Some(Direct(XMM8)), operand4: Some(Literal8(69)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 211, 213, 148, 85, 208, 69], OperandSize::Qword)
}

#[test]
fn vfixupimmsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMSD, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM30)), operand3: Some(IndirectDisplaced(RSI, 892312126, Some(OperandSize::Qword), None)), operand4: Some(Literal8(12)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 115, 141, 132, 85, 158, 62, 154, 47, 53, 12], OperandSize::Qword)
}

