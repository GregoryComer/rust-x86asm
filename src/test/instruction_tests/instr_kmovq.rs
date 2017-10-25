use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn kmovq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVQ, operand1: Some(Direct(K3)), operand2: Some(Direct(K3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 248, 144, 219], OperandSize::Dword)
}

#[test]
fn kmovq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVQ, operand1: Some(Direct(K7)), operand2: Some(IndirectScaledDisplaced(ESI, Eight, 1602891596, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 248, 144, 60, 245, 76, 47, 138, 95], OperandSize::Dword)
}

#[test]
fn kmovq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVQ, operand1: Some(Direct(K2)), operand2: Some(Direct(K7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 248, 144, 215], OperandSize::Qword)
}

#[test]
fn kmovq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVQ, operand1: Some(Direct(K5)), operand2: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 248, 144, 47], OperandSize::Qword)
}

#[test]
fn kmovq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVQ, operand1: Some(IndirectScaledDisplaced(EBX, Two, 100201132, Some(OperandSize::Qword), None)), operand2: Some(Direct(K5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 248, 145, 44, 93, 172, 242, 248, 5], OperandSize::Dword)
}

#[test]
fn kmovq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVQ, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Two, 287267972, Some(OperandSize::Qword), None)), operand2: Some(Direct(K7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 248, 145, 188, 65, 132, 92, 31, 17], OperandSize::Qword)
}

#[test]
fn kmovq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVQ, operand1: Some(Direct(K3)), operand2: Some(Direct(RBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 251, 146, 219], OperandSize::Qword)
}

#[test]
fn kmovq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVQ, operand1: Some(Direct(RBX)), operand2: Some(Direct(K2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 251, 147, 218], OperandSize::Qword)
}

