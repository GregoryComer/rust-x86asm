use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn kmovd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVD, operand1: Some(Direct(K4)), operand2: Some(Direct(K6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 249, 144, 230], OperandSize::Dword)
}

#[test]
fn kmovd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVD, operand1: Some(Direct(K1)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Four, 2053939829, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 249, 144, 140, 135, 117, 162, 108, 122], OperandSize::Dword)
}

#[test]
fn kmovd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVD, operand1: Some(Direct(K7)), operand2: Some(Direct(K7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 249, 144, 255], OperandSize::Qword)
}

#[test]
fn kmovd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVD, operand1: Some(Direct(K6)), operand2: Some(IndirectScaledIndexed(RSI, RDI, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 249, 144, 52, 190], OperandSize::Qword)
}

#[test]
fn kmovd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVD, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Four, 1386330266, Some(OperandSize::Dword), None)), operand2: Some(Direct(K4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 249, 145, 164, 191, 154, 184, 161, 82], OperandSize::Dword)
}

#[test]
fn kmovd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVD, operand1: Some(IndirectScaledIndexed(RSI, RCX, Four, Some(OperandSize::Dword), None)), operand2: Some(Direct(K7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 249, 145, 60, 142], OperandSize::Qword)
}

#[test]
fn kmovd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVD, operand1: Some(Direct(K1)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 146, 205], OperandSize::Dword)
}

#[test]
fn kmovd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVD, operand1: Some(Direct(K3)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 146, 222], OperandSize::Qword)
}

#[test]
fn kmovd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVD, operand1: Some(Direct(EDI)), operand2: Some(Direct(K6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 147, 254], OperandSize::Dword)
}

#[test]
fn kmovd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVD, operand1: Some(Direct(EDX)), operand2: Some(Direct(K4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 147, 212], OperandSize::Qword)
}

