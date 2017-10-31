use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn kmovq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVQ, operand1: Some(Direct(K1)), operand2: Some(Direct(K7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 248, 144, 207], OperandSize::Dword)
}

#[test]
fn kmovq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVQ, operand1: Some(Direct(K5)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 1436919726, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 248, 144, 44, 69, 174, 167, 165, 85], OperandSize::Dword)
}

#[test]
fn kmovq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVQ, operand1: Some(Direct(K6)), operand2: Some(Direct(K1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 248, 144, 241], OperandSize::Qword)
}

#[test]
fn kmovq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVQ, operand1: Some(Direct(K3)), operand2: Some(IndirectDisplaced(RCX, 882450793, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 248, 144, 153, 105, 33, 153, 52], OperandSize::Qword)
}

#[test]
fn kmovq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVQ, operand1: Some(IndirectScaledIndexed(ESI, ECX, Eight, Some(OperandSize::Qword), None)), operand2: Some(Direct(K6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 248, 145, 52, 206], OperandSize::Dword)
}

#[test]
fn kmovq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVQ, operand1: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand2: Some(Direct(K7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 248, 145, 56], OperandSize::Qword)
}

#[test]
fn kmovq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVQ, operand1: Some(Direct(K1)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 251, 146, 207], OperandSize::Qword)
}

#[test]
fn kmovq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVQ, operand1: Some(Direct(RDX)), operand2: Some(Direct(K2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 251, 147, 210], OperandSize::Qword)
}

