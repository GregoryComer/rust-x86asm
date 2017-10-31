use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movhps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVHPS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(EDX, Two, 994779988, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 22, 12, 85, 84, 35, 75, 59], OperandSize::Dword)
}

#[test]
fn movhps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVHPS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(RBX, Eight, 267702062, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 22, 12, 221, 46, 207, 244, 15], OperandSize::Qword)
}

#[test]
fn movhps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVHPS, operand1: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Eight, 547745916, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 23, 140, 241, 124, 240, 165, 32], OperandSize::Dword)
}

#[test]
fn movhps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVHPS, operand1: Some(IndirectDisplaced(RSI, 1229563122, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 23, 158, 242, 164, 73, 73], OperandSize::Qword)
}

