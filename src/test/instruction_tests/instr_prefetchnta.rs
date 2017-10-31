use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn prefetchnta_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHNTA, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 19945, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 24, 128, 233, 77], OperandSize::Word)
}

#[test]
fn prefetchnta_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHNTA, operand1: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Two, 1555778147, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 24, 132, 74, 99, 74, 187, 92], OperandSize::Dword)
}

#[test]
fn prefetchnta_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHNTA, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Two, 89916555, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 24, 132, 123, 139, 4, 92, 5], OperandSize::Qword)
}

