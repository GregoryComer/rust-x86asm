use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn blsr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSR, operand1: Some(Direct(EDX)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 104, 243, 205], OperandSize::Dword)
}

#[test]
fn blsr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSR, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledDisplaced(EBX, Four, 1423880418, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 64, 243, 12, 157, 226, 176, 222, 84], OperandSize::Dword)
}

#[test]
fn blsr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSR, operand1: Some(Direct(EDI)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 64, 243, 204], OperandSize::Qword)
}

#[test]
fn blsr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSR, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 1315305234, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 72, 243, 12, 69, 18, 247, 101, 78], OperandSize::Qword)
}

#[test]
fn blsr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSR, operand1: Some(Direct(RSP)), operand2: Some(Direct(RSI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 216, 243, 206], OperandSize::Qword)
}

#[test]
fn blsr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSR, operand1: Some(Direct(RDI)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Two, 903879624, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 192, 243, 140, 72, 200, 27, 224, 53], OperandSize::Qword)
}

