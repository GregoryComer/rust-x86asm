use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn kmovd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVD, operand1: Some(Direct(K1)), operand2: Some(Direct(K4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 249, 144, 204], OperandSize::Dword)
}

#[test]
fn kmovd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVD, operand1: Some(Direct(K5)), operand2: Some(IndirectScaledIndexed(ECX, EAX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 249, 144, 44, 65], OperandSize::Dword)
}

#[test]
fn kmovd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVD, operand1: Some(Direct(K2)), operand2: Some(Direct(K4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 249, 144, 212], OperandSize::Qword)
}

#[test]
fn kmovd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVD, operand1: Some(Direct(K3)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Eight, 1039481823, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 249, 144, 156, 223, 223, 59, 245, 61], OperandSize::Qword)
}

#[test]
fn kmovd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVD, operand1: Some(IndirectScaledIndexed(ECX, EDX, Two, Some(OperandSize::Dword), None)), operand2: Some(Direct(K4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 249, 145, 36, 81], OperandSize::Dword)
}

#[test]
fn kmovd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVD, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Eight, 1403597773, Some(OperandSize::Dword), None)), operand2: Some(Direct(K5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 249, 145, 172, 199, 205, 51, 169, 83], OperandSize::Qword)
}

#[test]
fn kmovd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVD, operand1: Some(Direct(K4)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 146, 230], OperandSize::Dword)
}

#[test]
fn kmovd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVD, operand1: Some(Direct(K1)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 146, 207], OperandSize::Qword)
}

#[test]
fn kmovd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVD, operand1: Some(Direct(EDI)), operand2: Some(Direct(K4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 147, 252], OperandSize::Dword)
}

#[test]
fn kmovd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVD, operand1: Some(Direct(EDI)), operand2: Some(Direct(K7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 147, 255], OperandSize::Qword)
}

