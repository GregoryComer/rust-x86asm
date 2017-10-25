use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn mulx_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MULX, operand1: Some(Direct(EBP)), operand2: Some(Direct(EDI)), operand3: Some(Direct(EDI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 67, 246, 239], OperandSize::Dword)
}

#[test]
fn mulx_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MULX, operand1: Some(Direct(EDI)), operand2: Some(Direct(EBX)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Eight, 726334288, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 99, 246, 188, 207, 80, 251, 74, 43], OperandSize::Dword)
}

#[test]
fn mulx_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MULX, operand1: Some(Direct(ECX)), operand2: Some(Direct(ECX)), operand3: Some(Direct(EDX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 115, 246, 202], OperandSize::Qword)
}

#[test]
fn mulx_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MULX, operand1: Some(Direct(ESI)), operand2: Some(Direct(EBP)), operand3: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 83, 246, 55], OperandSize::Qword)
}

#[test]
fn mulx_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MULX, operand1: Some(Direct(RSP)), operand2: Some(Direct(RSI)), operand3: Some(Direct(RSI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 203, 246, 230], OperandSize::Qword)
}

#[test]
fn mulx_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MULX, operand1: Some(Direct(RDX)), operand2: Some(Direct(RSI)), operand3: Some(IndirectDisplaced(RDX, 2005129220, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 203, 246, 146, 4, 216, 131, 119], OperandSize::Qword)
}

