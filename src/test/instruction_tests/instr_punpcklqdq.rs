use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn punpcklqdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLQDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 108, 242], OperandSize::Dword)
}

#[test]
fn punpcklqdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLQDQ, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(EBX, Four, 1723010695, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 108, 44, 157, 135, 14, 179, 102], OperandSize::Dword)
}

#[test]
fn punpcklqdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLQDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 108, 224], OperandSize::Qword)
}

#[test]
fn punpcklqdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLQDQ, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(RDI, RBX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 108, 4, 95], OperandSize::Qword)
}

