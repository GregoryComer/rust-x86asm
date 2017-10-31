use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmuldq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 40, 249], OperandSize::Dword)
}

#[test]
fn pmuldq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULDQ, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(EBX, Two, 340523591, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 40, 12, 93, 71, 250, 75, 20], OperandSize::Dword)
}

#[test]
fn pmuldq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULDQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 40, 236], OperandSize::Qword)
}

#[test]
fn pmuldq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULDQ, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Eight, 958036779, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 40, 180, 216, 43, 123, 26, 57], OperandSize::Qword)
}

