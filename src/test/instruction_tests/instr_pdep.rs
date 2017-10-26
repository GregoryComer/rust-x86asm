use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pdep_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PDEP, operand1: Some(Direct(ECX)), operand2: Some(Direct(ECX)), operand3: Some(Direct(EBP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 115, 245, 205], OperandSize::Dword)
}

#[test]
fn pdep_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PDEP, operand1: Some(Direct(ESP)), operand2: Some(Direct(EDI)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 1682950300, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 67, 245, 36, 117, 156, 200, 79, 100], OperandSize::Dword)
}

#[test]
fn pdep_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PDEP, operand1: Some(Direct(EDI)), operand2: Some(Direct(ECX)), operand3: Some(Direct(EBP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 115, 245, 253], OperandSize::Qword)
}

#[test]
fn pdep_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PDEP, operand1: Some(Direct(EBP)), operand2: Some(Direct(EBP)), operand3: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 83, 245, 43], OperandSize::Qword)
}

#[test]
fn pdep_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PDEP, operand1: Some(Direct(RDX)), operand2: Some(Direct(RDX)), operand3: Some(Direct(RBP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 235, 245, 213], OperandSize::Qword)
}

#[test]
fn pdep_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PDEP, operand1: Some(Direct(RDI)), operand2: Some(Direct(RSI)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 1969454916, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 203, 245, 60, 181, 68, 127, 99, 117], OperandSize::Qword)
}

