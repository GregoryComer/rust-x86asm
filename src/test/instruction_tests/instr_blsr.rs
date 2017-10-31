use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn blsr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSR, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 72, 243, 207], OperandSize::Dword)
}

#[test]
fn blsr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSR, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledDisplaced(EBX, Eight, 1072429243, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 112, 243, 12, 221, 187, 248, 235, 63], OperandSize::Dword)
}

#[test]
fn blsr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSR, operand1: Some(Direct(EDX)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 104, 243, 205], OperandSize::Qword)
}

#[test]
fn blsr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSR, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexed(RSI, RDI, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 96, 243, 12, 126], OperandSize::Qword)
}

#[test]
fn blsr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSR, operand1: Some(Direct(RSP)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 216, 243, 205], OperandSize::Qword)
}

#[test]
fn blsr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSR, operand1: Some(Direct(RDI)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 746619833, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 192, 243, 12, 69, 185, 131, 128, 44], OperandSize::Qword)
}

