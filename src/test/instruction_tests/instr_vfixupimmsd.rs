use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfixupimmsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: Some(Literal8(19)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 245, 158, 85, 200, 19], OperandSize::Dword)
}

#[test]
fn vfixupimmsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 682690048, Some(OperandSize::Qword), None)), operand4: Some(Literal8(82)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 221, 137, 85, 36, 85, 0, 6, 177, 40, 82], OperandSize::Dword)
}

#[test]
fn vfixupimmsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM29)), operand3: Some(Direct(XMM29)), operand4: Some(Literal8(70)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 147, 149, 147, 85, 253, 70], OperandSize::Qword)
}

#[test]
fn vfixupimmsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMSD, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM12)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Eight, 2038373636, Some(OperandSize::Qword), None)), operand4: Some(Literal8(111)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 99, 157, 143, 85, 180, 214, 4, 29, 127, 121, 111], OperandSize::Qword)
}

