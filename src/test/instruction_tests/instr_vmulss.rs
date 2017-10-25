use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmulss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 89, 234], OperandSize::Dword)
}

#[test]
fn vmulss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(EDX, 49781454, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 242, 89, 130, 206, 154, 247, 2], OperandSize::Dword)
}

#[test]
fn vmulss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 89, 222], OperandSize::Qword)
}

#[test]
fn vmulss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 218, 89, 10], OperandSize::Qword)
}

#[test]
fn vmulss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 118, 188, 89, 209], OperandSize::Dword)
}

#[test]
fn vmulss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 306777469, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 86, 138, 89, 60, 93, 125, 13, 73, 18], OperandSize::Dword)
}

#[test]
fn vmulss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM24)), operand3: Some(Direct(XMM11)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 209, 62, 183, 89, 195], OperandSize::Qword)
}

#[test]
fn vmulss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSS, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 137015107, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 86, 141, 89, 4, 69, 67, 175, 42, 8], OperandSize::Qword)
}

