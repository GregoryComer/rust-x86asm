use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfixupimmss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(44)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 69, 156, 85, 196, 44], OperandSize::Dword)
}

#[test]
fn vfixupimmss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand4: Some(Literal8(35)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 109, 143, 85, 63, 35], OperandSize::Dword)
}

#[test]
fn vfixupimmss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMSS, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(89)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 115, 93, 157, 85, 204, 89], OperandSize::Qword)
}

#[test]
fn vfixupimmss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand4: Some(Literal8(72)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 101, 142, 85, 50, 72], OperandSize::Qword)
}

