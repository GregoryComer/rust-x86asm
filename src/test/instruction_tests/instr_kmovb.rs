use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn kmovb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVB, operand1: Some(Direct(K1)), operand2: Some(Direct(K3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 144, 203], OperandSize::Dword)
}

#[test]
fn kmovb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVB, operand1: Some(Direct(K4)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Four, 774126646, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 144, 164, 134, 54, 60, 36, 46], OperandSize::Dword)
}

#[test]
fn kmovb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVB, operand1: Some(Direct(K2)), operand2: Some(Direct(K1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 144, 209], OperandSize::Qword)
}

#[test]
fn kmovb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVB, operand1: Some(Direct(K4)), operand2: Some(IndirectScaledIndexed(RDX, RBX, Eight, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 144, 36, 218], OperandSize::Qword)
}

#[test]
fn kmovb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVB, operand1: Some(Indirect(ECX, Some(OperandSize::Byte), None)), operand2: Some(Direct(K6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 145, 49], OperandSize::Dword)
}

#[test]
fn kmovb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVB, operand1: Some(Indirect(RAX, Some(OperandSize::Byte), None)), operand2: Some(Direct(K7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 145, 56], OperandSize::Qword)
}

#[test]
fn kmovb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVB, operand1: Some(Direct(K4)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 146, 231], OperandSize::Dword)
}

#[test]
fn kmovb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVB, operand1: Some(Direct(K6)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 146, 244], OperandSize::Qword)
}

#[test]
fn kmovb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVB, operand1: Some(Direct(ESP)), operand2: Some(Direct(K7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 147, 231], OperandSize::Dword)
}

#[test]
fn kmovb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVB, operand1: Some(Direct(ECX)), operand2: Some(Direct(K4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 147, 204], OperandSize::Qword)
}

