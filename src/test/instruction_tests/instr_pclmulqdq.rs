use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pclmulqdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCLMULQDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(10)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 68, 220, 10], OperandSize::Dword)
}

#[test]
fn pclmulqdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCLMULQDQ, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(EDI, 1774258621, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(104)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 68, 183, 189, 9, 193, 105, 104], OperandSize::Dword)
}

#[test]
fn pclmulqdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCLMULQDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(40)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 68, 247, 40], OperandSize::Qword)
}

#[test]
fn pclmulqdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCLMULQDQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 1082964066, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 68, 60, 213, 98, 184, 140, 64, 3], OperandSize::Qword)
}

