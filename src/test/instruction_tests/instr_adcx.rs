use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn adcx_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ADCX, operand1: Some(Direct(ESP)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 246, 231], OperandSize::Dword)
}

#[test]
fn adcx_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ADCX, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Eight, 1078041768, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 246, 164, 251, 168, 156, 65, 64], OperandSize::Dword)
}

#[test]
fn adcx_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ADCX, operand1: Some(Direct(ECX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 246, 201], OperandSize::Qword)
}

#[test]
fn adcx_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ADCX, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Eight, 171220487, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 246, 148, 194, 7, 158, 52, 10], OperandSize::Qword)
}

#[test]
fn adcx_5() {
    run_test(&Instruction { mnemonic: Mnemonic::ADCX, operand1: Some(Direct(RBP)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 72, 15, 56, 246, 236], OperandSize::Qword)
}

#[test]
fn adcx_6() {
    run_test(&Instruction { mnemonic: Mnemonic::ADCX, operand1: Some(Direct(RBP)), operand2: Some(IndirectScaledIndexed(RDI, RDI, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 72, 15, 56, 246, 44, 191], OperandSize::Qword)
}

