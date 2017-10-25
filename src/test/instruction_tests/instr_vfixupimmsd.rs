use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfixupimmsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM6)), operand4: Some(Literal8(51)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 229, 153, 85, 206, 51], OperandSize::Dword)
}

#[test]
fn vfixupimmsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Two, 1772549057, Some(OperandSize::Qword), None)), operand4: Some(Literal8(90)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 213, 141, 85, 188, 70, 193, 243, 166, 105, 90], OperandSize::Dword)
}

#[test]
fn vfixupimmsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMSD, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM22)), operand3: Some(Direct(XMM10)), operand4: Some(Literal8(2)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 83, 205, 151, 85, 234, 2], OperandSize::Qword)
}

#[test]
fn vfixupimmsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMSD, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(RCX, RSI, Eight, Some(OperandSize::Qword), None)), operand4: Some(Literal8(87)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 99, 253, 139, 85, 28, 241, 87], OperandSize::Qword)
}

