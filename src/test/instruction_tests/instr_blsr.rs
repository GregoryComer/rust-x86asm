use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn blsr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSR, operand1: Some(Direct(ECX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 112, 243, 207], OperandSize::Dword)
}

#[test]
fn blsr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSR, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Two, 1115484477, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 112, 243, 140, 87, 61, 241, 124, 66], OperandSize::Dword)
}

#[test]
fn blsr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSR, operand1: Some(Direct(EBP)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 80, 243, 204], OperandSize::Qword)
}

#[test]
fn blsr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSR, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledDisplaced(RDI, Four, 1972910153, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 104, 243, 12, 189, 73, 56, 152, 117], OperandSize::Qword)
}

#[test]
fn blsr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSR, operand1: Some(Direct(RSP)), operand2: Some(Direct(RCX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 216, 243, 201], OperandSize::Qword)
}

#[test]
fn blsr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSR, operand1: Some(Direct(RDI)), operand2: Some(IndirectScaledIndexed(RSI, RDX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 192, 243, 12, 86], OperandSize::Qword)
}

