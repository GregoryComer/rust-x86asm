use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmulss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 202, 89, 251], OperandSize::Dword)
}

#[test]
fn vmulss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 1873354912, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 218, 89, 52, 213, 160, 32, 169, 111], OperandSize::Dword)
}

#[test]
fn vmulss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 242, 89, 203], OperandSize::Qword)
}

#[test]
fn vmulss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 234, 89, 42], OperandSize::Qword)
}

#[test]
fn vmulss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 70, 223, 89, 201], OperandSize::Dword)
}

#[test]
fn vmulss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 94, 138, 89, 25], OperandSize::Dword)
}

#[test]
fn vmulss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSS, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM8)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 62, 190, 89, 195], OperandSize::Qword)
}

#[test]
fn vmulss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSS, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM25)), operand3: Some(IndirectDisplaced(RBX, 1846244113, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 54, 130, 89, 155, 17, 115, 11, 110], OperandSize::Qword)
}

