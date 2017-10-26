use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmovsxdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXDQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 37, 235], OperandSize::Dword)
}

#[test]
fn pmovsxdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXDQ, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 37, 1], OperandSize::Dword)
}

#[test]
fn pmovsxdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 37, 198], OperandSize::Qword)
}

#[test]
fn pmovsxdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXDQ, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(RAX, 1225922296, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 37, 128, 248, 22, 18, 73], OperandSize::Qword)
}

