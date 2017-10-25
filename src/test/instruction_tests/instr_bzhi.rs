use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn bzhi_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BZHI, operand1: Some(Direct(EBP)), operand2: Some(Direct(ESP)), operand3: Some(Direct(EDI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 64, 245, 236], OperandSize::Dword)
}

#[test]
fn bzhi_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BZHI, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexed(EDX, EBX, Eight, Some(OperandSize::Dword), None)), operand3: Some(Direct(EBP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 80, 245, 44, 218], OperandSize::Dword)
}

#[test]
fn bzhi_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BZHI, operand1: Some(Direct(ESI)), operand2: Some(Direct(ESI)), operand3: Some(Direct(EBP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 80, 245, 246], OperandSize::Qword)
}

#[test]
fn bzhi_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BZHI, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexed(RCX, RBX, Two, Some(OperandSize::Dword), None)), operand3: Some(Direct(ESI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 72, 245, 44, 89], OperandSize::Qword)
}

#[test]
fn bzhi_5() {
    run_test(&Instruction { mnemonic: Mnemonic::BZHI, operand1: Some(Direct(RSI)), operand2: Some(Direct(RDX)), operand3: Some(Direct(RDX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 232, 245, 242], OperandSize::Qword)
}

#[test]
fn bzhi_6() {
    run_test(&Instruction { mnemonic: Mnemonic::BZHI, operand1: Some(Direct(RBX)), operand2: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand3: Some(Direct(RSI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 200, 245, 31], OperandSize::Qword)
}

