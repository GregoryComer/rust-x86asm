use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn adcx_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ADCX, operand1: Some(Direct(EBX)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 246, 222], OperandSize::Dword)
}

#[test]
fn adcx_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ADCX, operand1: Some(Direct(ECX)), operand2: Some(IndirectDisplaced(EDI, 1964182472, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 246, 143, 200, 11, 19, 117], OperandSize::Dword)
}

#[test]
fn adcx_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ADCX, operand1: Some(Direct(ECX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 246, 204], OperandSize::Qword)
}

#[test]
fn adcx_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ADCX, operand1: Some(Direct(ESI)), operand2: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 246, 54], OperandSize::Qword)
}

#[test]
fn adcx_5() {
    run_test(&Instruction { mnemonic: Mnemonic::ADCX, operand1: Some(Direct(RCX)), operand2: Some(Direct(RSI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 72, 15, 56, 246, 206], OperandSize::Qword)
}

#[test]
fn adcx_6() {
    run_test(&Instruction { mnemonic: Mnemonic::ADCX, operand1: Some(Direct(RDI)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Two, 1824843560, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 72, 15, 56, 246, 188, 82, 40, 231, 196, 108], OperandSize::Qword)
}

