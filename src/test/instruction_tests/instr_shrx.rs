use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn shrx_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRX, operand1: Some(Direct(EBP)), operand2: Some(Direct(EBX)), operand3: Some(Direct(EBP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 83, 247, 235], OperandSize::Dword)
}

#[test]
fn shrx_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRX, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Four, 1551320053, Some(OperandSize::Dword), None)), operand3: Some(Direct(EBX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 99, 247, 156, 154, 245, 67, 119, 92], OperandSize::Dword)
}

#[test]
fn shrx_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRX, operand1: Some(Direct(ECX)), operand2: Some(Direct(EBP)), operand3: Some(Direct(EBX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 99, 247, 205], OperandSize::Qword)
}

#[test]
fn shrx_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRX, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Two, 576760328, Some(OperandSize::Dword), None)), operand3: Some(Direct(EBP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 83, 247, 148, 112, 8, 170, 96, 34], OperandSize::Qword)
}

#[test]
fn shrx_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRX, operand1: Some(Direct(RSP)), operand2: Some(Direct(RBX)), operand3: Some(Direct(RBP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 211, 247, 227], OperandSize::Qword)
}

#[test]
fn shrx_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRX, operand1: Some(Direct(RBX)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 186228201, Some(OperandSize::Qword), None)), operand3: Some(Direct(RDI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 195, 247, 28, 149, 233, 157, 25, 11], OperandSize::Qword)
}

