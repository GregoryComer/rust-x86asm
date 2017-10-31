use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn adcx_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ADCX, operand1: Some(Direct(EBP)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 246, 233], OperandSize::Dword)
}

#[test]
fn adcx_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ADCX, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexed(EDX, ECX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 246, 60, 202], OperandSize::Dword)
}

#[test]
fn adcx_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ADCX, operand1: Some(Direct(ESP)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 246, 227], OperandSize::Qword)
}

#[test]
fn adcx_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ADCX, operand1: Some(Direct(ESP)), operand2: Some(IndirectDisplaced(RAX, 1437549993, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 246, 160, 169, 69, 175, 85], OperandSize::Qword)
}

#[test]
fn adcx_5() {
    run_test(&Instruction { mnemonic: Mnemonic::ADCX, operand1: Some(Direct(RBP)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 72, 15, 56, 246, 237], OperandSize::Qword)
}

#[test]
fn adcx_6() {
    run_test(&Instruction { mnemonic: Mnemonic::ADCX, operand1: Some(Direct(RCX)), operand2: Some(IndirectDisplaced(RBX, 494377971, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 72, 15, 56, 246, 139, 243, 155, 119, 29], OperandSize::Qword)
}

