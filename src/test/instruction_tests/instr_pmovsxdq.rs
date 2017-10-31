use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmovsxdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 37, 222], OperandSize::Dword)
}

#[test]
fn pmovsxdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXDQ, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(EDX, 430274183, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 37, 154, 135, 118, 165, 25], OperandSize::Dword)
}

#[test]
fn pmovsxdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 37, 221], OperandSize::Qword)
}

#[test]
fn pmovsxdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXDQ, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(RAX, RBX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 37, 44, 88], OperandSize::Qword)
}

