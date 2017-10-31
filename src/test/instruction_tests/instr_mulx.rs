use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn mulx_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MULX, operand1: Some(Direct(ECX)), operand2: Some(Direct(ESP)), operand3: Some(Direct(EDX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 91, 246, 202], OperandSize::Dword)
}

#[test]
fn mulx_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MULX, operand1: Some(Direct(EBP)), operand2: Some(Direct(EBX)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 620740657, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 99, 246, 44, 77, 49, 192, 255, 36], OperandSize::Dword)
}

#[test]
fn mulx_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MULX, operand1: Some(Direct(EBP)), operand2: Some(Direct(ESI)), operand3: Some(Direct(EBX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 75, 246, 235], OperandSize::Qword)
}

#[test]
fn mulx_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MULX, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDI)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Four, 121392359, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 67, 246, 180, 143, 231, 76, 60, 7], OperandSize::Qword)
}

#[test]
fn mulx_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MULX, operand1: Some(Direct(RSP)), operand2: Some(Direct(RBP)), operand3: Some(Direct(RDX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 211, 246, 226], OperandSize::Qword)
}

#[test]
fn mulx_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MULX, operand1: Some(Direct(RSP)), operand2: Some(Direct(RCX)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Four, 812365078, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 243, 246, 164, 183, 22, 181, 107, 48], OperandSize::Qword)
}

