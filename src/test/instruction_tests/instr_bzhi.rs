use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn bzhi_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BZHI, operand1: Some(Direct(EDI)), operand2: Some(Direct(ESI)), operand3: Some(Direct(EDI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 64, 245, 254], OperandSize::Dword)
}

#[test]
fn bzhi_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BZHI, operand1: Some(Direct(EBP)), operand2: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand3: Some(Direct(EBX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 96, 245, 40], OperandSize::Dword)
}

#[test]
fn bzhi_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BZHI, operand1: Some(Direct(EDX)), operand2: Some(Direct(ECX)), operand3: Some(Direct(EBP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 80, 245, 209], OperandSize::Qword)
}

#[test]
fn bzhi_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BZHI, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexed(RSI, RAX, Two, Some(OperandSize::Dword), None)), operand3: Some(Direct(EDX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 104, 245, 28, 70], OperandSize::Qword)
}

#[test]
fn bzhi_5() {
    run_test(&Instruction { mnemonic: Mnemonic::BZHI, operand1: Some(Direct(RSP)), operand2: Some(Direct(RBP)), operand3: Some(Direct(RDI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 192, 245, 229], OperandSize::Qword)
}

#[test]
fn bzhi_6() {
    run_test(&Instruction { mnemonic: Mnemonic::BZHI, operand1: Some(Direct(RBX)), operand2: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand3: Some(Direct(RCX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 240, 245, 27], OperandSize::Qword)
}

