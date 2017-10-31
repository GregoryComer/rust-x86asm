use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pclmulqdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCLMULQDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(71)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 68, 211, 71], OperandSize::Dword)
}

#[test]
fn pclmulqdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCLMULQDQ, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 1227239826, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(94)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 68, 4, 77, 146, 49, 38, 73, 94], OperandSize::Dword)
}

#[test]
fn pclmulqdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCLMULQDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(95)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 68, 221, 95], OperandSize::Qword)
}

#[test]
fn pclmulqdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCLMULQDQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(RCX, RSI, Two, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(56)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 68, 36, 113, 56], OperandSize::Qword)
}

