use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn mulx_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MULX, operand1: Some(Direct(EBP)), operand2: Some(Direct(ESP)), operand3: Some(Direct(EBX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 91, 246, 235], OperandSize::Dword)
}

#[test]
fn mulx_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MULX, operand1: Some(Direct(EDX)), operand2: Some(Direct(ESP)), operand3: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 91, 246, 19], OperandSize::Dword)
}

#[test]
fn mulx_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MULX, operand1: Some(Direct(ESI)), operand2: Some(Direct(ESI)), operand3: Some(Direct(EDI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 75, 246, 247], OperandSize::Qword)
}

#[test]
fn mulx_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MULX, operand1: Some(Direct(EDX)), operand2: Some(Direct(EDI)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Eight, 1652058613, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 67, 246, 148, 247, 245, 105, 120, 98], OperandSize::Qword)
}

#[test]
fn mulx_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MULX, operand1: Some(Direct(RCX)), operand2: Some(Direct(RBX)), operand3: Some(Direct(RBP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 227, 246, 205], OperandSize::Qword)
}

#[test]
fn mulx_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MULX, operand1: Some(Direct(RDI)), operand2: Some(Direct(RCX)), operand3: Some(IndirectScaledIndexed(RAX, RBX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 243, 246, 60, 216], OperandSize::Qword)
}

