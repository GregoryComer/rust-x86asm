use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn kmovq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVQ, operand1: Some(Direct(K3)), operand2: Some(Direct(K1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 248, 144, 217], OperandSize::Dword)
}

#[test]
fn kmovq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVQ, operand1: Some(Direct(K6)), operand2: Some(IndirectScaledDisplaced(EBX, Four, 269460094, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 248, 144, 52, 157, 126, 162, 15, 16], OperandSize::Dword)
}

#[test]
fn kmovq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVQ, operand1: Some(Direct(K7)), operand2: Some(Direct(K7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 248, 144, 255], OperandSize::Qword)
}

#[test]
fn kmovq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVQ, operand1: Some(Direct(K6)), operand2: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 248, 144, 49], OperandSize::Qword)
}

#[test]
fn kmovq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVQ, operand1: Some(IndirectScaledDisplaced(EDX, Four, 1454282932, Some(OperandSize::Qword), None)), operand2: Some(Direct(K7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 248, 145, 60, 149, 180, 152, 174, 86], OperandSize::Dword)
}

#[test]
fn kmovq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVQ, operand1: Some(IndirectScaledIndexed(RDI, RSI, Two, Some(OperandSize::Qword), None)), operand2: Some(Direct(K5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 248, 145, 44, 119], OperandSize::Qword)
}

#[test]
fn kmovq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVQ, operand1: Some(Direct(K7)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 251, 146, 252], OperandSize::Qword)
}

#[test]
fn kmovq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVQ, operand1: Some(Direct(RSP)), operand2: Some(Direct(K2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 251, 147, 226], OperandSize::Qword)
}

