use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn kmovb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVB, operand1: Some(Direct(K4)), operand2: Some(Direct(K5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 144, 229], OperandSize::Dword)
}

#[test]
fn kmovb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVB, operand1: Some(Direct(K1)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Eight, 369502121, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 144, 140, 246, 169, 39, 6, 22], OperandSize::Dword)
}

#[test]
fn kmovb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVB, operand1: Some(Direct(K5)), operand2: Some(Direct(K4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 144, 236], OperandSize::Qword)
}

#[test]
fn kmovb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVB, operand1: Some(Direct(K5)), operand2: Some(IndirectScaledDisplaced(RSI, Four, 1572027351, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 144, 44, 181, 215, 59, 179, 93], OperandSize::Qword)
}

#[test]
fn kmovb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVB, operand1: Some(Indirect(EDI, Some(OperandSize::Byte), None)), operand2: Some(Direct(K5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 145, 47], OperandSize::Dword)
}

#[test]
fn kmovb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVB, operand1: Some(Indirect(RSI, Some(OperandSize::Byte), None)), operand2: Some(Direct(K3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 145, 30], OperandSize::Qword)
}

#[test]
fn kmovb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVB, operand1: Some(Direct(K6)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 146, 244], OperandSize::Dword)
}

#[test]
fn kmovb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVB, operand1: Some(Direct(K2)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 146, 211], OperandSize::Qword)
}

#[test]
fn kmovb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVB, operand1: Some(Direct(EDI)), operand2: Some(Direct(K3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 147, 251], OperandSize::Dword)
}

#[test]
fn kmovb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVB, operand1: Some(Direct(ESP)), operand2: Some(Direct(K5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 147, 229], OperandSize::Qword)
}

