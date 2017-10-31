use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn punpcklqdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLQDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 108, 218], OperandSize::Dword)
}

#[test]
fn punpcklqdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLQDQ, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 1966139335, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 108, 28, 197, 199, 231, 48, 117], OperandSize::Dword)
}

#[test]
fn punpcklqdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLQDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 108, 252], OperandSize::Qword)
}

#[test]
fn punpcklqdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLQDQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Eight, 616236116, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 108, 188, 215, 84, 4, 187, 36], OperandSize::Qword)
}

