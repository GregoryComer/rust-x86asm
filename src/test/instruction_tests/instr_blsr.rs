use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn blsr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSR, operand1: Some(Direct(EBP)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 80, 243, 204], OperandSize::Dword)
}

#[test]
fn blsr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSR, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Eight, 1092839946, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 80, 243, 140, 222, 10, 106, 35, 65], OperandSize::Dword)
}

#[test]
fn blsr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSR, operand1: Some(Direct(EBP)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 80, 243, 204], OperandSize::Qword)
}

#[test]
fn blsr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSR, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexed(RDI, RAX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 112, 243, 12, 71], OperandSize::Qword)
}

#[test]
fn blsr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSR, operand1: Some(Direct(RSP)), operand2: Some(Direct(RBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 216, 243, 203], OperandSize::Qword)
}

#[test]
fn blsr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSR, operand1: Some(Direct(RCX)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Eight, 735401639, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 240, 243, 140, 218, 167, 86, 213, 43], OperandSize::Qword)
}

