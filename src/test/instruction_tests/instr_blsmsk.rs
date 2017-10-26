use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn blsmsk_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSMSK, operand1: Some(Direct(EBX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 96, 243, 212], OperandSize::Dword)
}

#[test]
fn blsmsk_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSMSK, operand1: Some(Direct(EBP)), operand2: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 80, 243, 16], OperandSize::Dword)
}

#[test]
fn blsmsk_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSMSK, operand1: Some(Direct(ECX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 112, 243, 209], OperandSize::Qword)
}

#[test]
fn blsmsk_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSMSK, operand1: Some(Direct(EDX)), operand2: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 104, 243, 23], OperandSize::Qword)
}

#[test]
fn blsmsk_5() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSMSK, operand1: Some(Direct(RSI)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 200, 243, 212], OperandSize::Qword)
}

#[test]
fn blsmsk_6() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSMSK, operand1: Some(Direct(RDI)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Eight, 960851900, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 192, 243, 148, 200, 188, 111, 69, 57], OperandSize::Qword)
}

