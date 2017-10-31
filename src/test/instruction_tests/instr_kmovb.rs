use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn kmovb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVB, operand1: Some(Direct(K7)), operand2: Some(Direct(K3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 144, 251], OperandSize::Dword)
}

#[test]
fn kmovb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVB, operand1: Some(Direct(K7)), operand2: Some(Indirect(EAX, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 144, 56], OperandSize::Dword)
}

#[test]
fn kmovb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVB, operand1: Some(Direct(K6)), operand2: Some(Direct(K4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 144, 244], OperandSize::Qword)
}

#[test]
fn kmovb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVB, operand1: Some(Direct(K5)), operand2: Some(IndirectScaledDisplaced(RBX, Two, 1949691258, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 144, 44, 93, 122, 237, 53, 116], OperandSize::Qword)
}

#[test]
fn kmovb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVB, operand1: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Eight, 1243483406, Some(OperandSize::Byte), None)), operand2: Some(Direct(K5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 145, 172, 202, 14, 13, 30, 74], OperandSize::Dword)
}

#[test]
fn kmovb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVB, operand1: Some(Indirect(RAX, Some(OperandSize::Byte), None)), operand2: Some(Direct(K2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 145, 16], OperandSize::Qword)
}

#[test]
fn kmovb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVB, operand1: Some(Direct(K4)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 146, 231], OperandSize::Dword)
}

#[test]
fn kmovb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVB, operand1: Some(Direct(K2)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 146, 215], OperandSize::Qword)
}

#[test]
fn kmovb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVB, operand1: Some(Direct(EBP)), operand2: Some(Direct(K7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 147, 239], OperandSize::Dword)
}

#[test]
fn kmovb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVB, operand1: Some(Direct(EDX)), operand2: Some(Direct(K1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 147, 209], OperandSize::Qword)
}

