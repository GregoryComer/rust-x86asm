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
    run_test(&Instruction { mnemonic: Mnemonic::KMOVB, operand1: Some(Direct(K7)), operand2: Some(IndirectScaledIndexed(EBX, EAX, Four, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 144, 60, 131], OperandSize::Dword)
}

#[test]
fn kmovb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVB, operand1: Some(Direct(K3)), operand2: Some(Direct(K6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 144, 222], OperandSize::Qword)
}

#[test]
fn kmovb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVB, operand1: Some(Direct(K1)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Four, 1617680417, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 144, 140, 155, 33, 216, 107, 96], OperandSize::Qword)
}

#[test]
fn kmovb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVB, operand1: Some(Indirect(ESI, Some(OperandSize::Byte), None)), operand2: Some(Direct(K5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 145, 46], OperandSize::Dword)
}

#[test]
fn kmovb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVB, operand1: Some(IndirectDisplaced(RDI, 199276451, Some(OperandSize::Byte), None)), operand2: Some(Direct(K2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 145, 151, 163, 183, 224, 11], OperandSize::Qword)
}

#[test]
fn kmovb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVB, operand1: Some(Direct(K1)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 146, 205], OperandSize::Dword)
}

#[test]
fn kmovb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVB, operand1: Some(Direct(K3)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 146, 220], OperandSize::Qword)
}

#[test]
fn kmovb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVB, operand1: Some(Direct(ESP)), operand2: Some(Direct(K7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 147, 231], OperandSize::Dword)
}

#[test]
fn kmovb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVB, operand1: Some(Direct(ESP)), operand2: Some(Direct(K3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 147, 227], OperandSize::Qword)
}

