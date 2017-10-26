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
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLQDQ, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 108, 39], OperandSize::Dword)
}

#[test]
fn punpcklqdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLQDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 108, 220], OperandSize::Qword)
}

#[test]
fn punpcklqdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLQDQ, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(RAX, RDX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 108, 28, 208], OperandSize::Qword)
}

