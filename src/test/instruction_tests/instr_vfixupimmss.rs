use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfixupimmss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: Some(Literal8(34)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 109, 155, 85, 214, 34], OperandSize::Dword)
}

#[test]
fn vfixupimmss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 86877561, Some(OperandSize::Dword), None)), operand4: Some(Literal8(29)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 85, 142, 85, 52, 69, 121, 165, 45, 5, 29], OperandSize::Dword)
}

#[test]
fn vfixupimmss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMSS, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM24)), operand3: Some(Direct(XMM3)), operand4: Some(Literal8(5)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 227, 61, 146, 85, 243, 5], OperandSize::Qword)
}

#[test]
fn vfixupimmss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMSS, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand4: Some(Literal8(65)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 115, 93, 142, 85, 50, 65], OperandSize::Qword)
}

