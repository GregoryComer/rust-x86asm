use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfixupimmsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(103)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 205, 158, 85, 226, 103], OperandSize::Dword)
}

#[test]
fn vfixupimmsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(EDX, EDI, Two, Some(OperandSize::Qword), None)), operand4: Some(Literal8(106)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 205, 141, 85, 28, 122, 106], OperandSize::Dword)
}

#[test]
fn vfixupimmsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMSD, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM14)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(31)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 227, 141, 154, 85, 218, 31], OperandSize::Qword)
}

#[test]
fn vfixupimmsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM26)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 1095357098, Some(OperandSize::Qword), None)), operand4: Some(Literal8(65)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 173, 129, 85, 36, 253, 170, 210, 73, 65, 65], OperandSize::Qword)
}

