use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfpclassss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSSS, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(73)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 125, 9, 103, 243, 73], OperandSize::Dword)
}

#[test]
fn vfpclassss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSSS, operand1: Some(Direct(K5)), operand2: Some(IndirectDisplaced(EBX, 1128128882, Some(OperandSize::Dword), None)), operand3: Some(Literal8(113)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 125, 12, 103, 171, 114, 225, 61, 67, 113], OperandSize::Dword)
}

#[test]
fn vfpclassss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSSS, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM27)), operand3: Some(Literal8(65)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 147, 125, 10, 103, 251, 65], OperandSize::Qword)
}

#[test]
fn vfpclassss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSSS, operand1: Some(Direct(K2)), operand2: Some(IndirectDisplaced(RAX, 613723747, Some(OperandSize::Dword), None)), operand3: Some(Literal8(97)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 125, 14, 103, 144, 99, 174, 148, 36, 97], OperandSize::Qword)
}

