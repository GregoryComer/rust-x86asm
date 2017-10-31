use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn punpcklqdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLQDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 108, 246], OperandSize::Dword)
}

#[test]
fn punpcklqdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLQDQ, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 108, 16], OperandSize::Dword)
}

#[test]
fn punpcklqdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLQDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 108, 252], OperandSize::Qword)
}

#[test]
fn punpcklqdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLQDQ, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(RDX, 1345978951, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 108, 138, 71, 2, 58, 80], OperandSize::Qword)
}

