use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vaddss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 242, 88, 223], OperandSize::Dword)
}

#[test]
fn vaddss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(EDI, 794158371, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 194, 88, 159, 35, 229, 85, 47], OperandSize::Dword)
}

#[test]
fn vaddss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 202, 88, 251], OperandSize::Qword)
}

#[test]
fn vaddss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 901414000, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 234, 88, 36, 85, 112, 124, 186, 53], OperandSize::Qword)
}

#[test]
fn vaddss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 118, 249, 88, 244], OperandSize::Dword)
}

#[test]
fn vaddss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 1944721856, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 110, 140, 88, 20, 117, 192, 25, 234, 115], OperandSize::Dword)
}

#[test]
fn vaddss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM18)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 177, 70, 154, 88, 202], OperandSize::Qword)
}

#[test]
fn vaddss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSS, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM11)), operand3: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 38, 137, 88, 2], OperandSize::Qword)
}

