use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmpss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(94)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 194, 196, 94], OperandSize::Dword)
}

#[test]
fn cmpss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPSS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(ECX, 712415121, Some(OperandSize::Dword), None)), operand3: Some(Literal8(19)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 194, 185, 145, 151, 118, 42, 19], OperandSize::Dword)
}

#[test]
fn cmpss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(11)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 194, 231, 11], OperandSize::Qword)
}

#[test]
fn cmpss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPSS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(RAX, 1640090676, Some(OperandSize::Dword), None)), operand3: Some(Literal8(26)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 194, 184, 52, 204, 193, 97, 26], OperandSize::Qword)
}

