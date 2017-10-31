use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfixupimmss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(74)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 109, 157, 85, 252, 74], OperandSize::Dword)
}

#[test]
fn vfixupimmss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 716225237, Some(OperandSize::Dword), None)), operand4: Some(Literal8(10)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 69, 139, 85, 44, 141, 213, 186, 176, 42, 10], OperandSize::Dword)
}

#[test]
fn vfixupimmss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMSS, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM10)), operand3: Some(Direct(XMM9)), operand4: Some(Literal8(20)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 195, 45, 159, 85, 241, 20], OperandSize::Qword)
}

#[test]
fn vfixupimmss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMSS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM27)), operand3: Some(IndirectScaledIndexed(RCX, RDI, Eight, Some(OperandSize::Dword), None)), operand4: Some(Literal8(61)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 37, 132, 85, 28, 249, 61], OperandSize::Qword)
}

