use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn lidt_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LIDT, operand1: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 24], OperandSize::Word)
}

#[test]
fn lidt_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LIDT, operand1: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Four, 1542439672, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 156, 142, 248, 194, 239, 91], OperandSize::Dword)
}

#[test]
fn lidt_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LIDT, operand1: Some(IndirectScaledDisplaced(RAX, Eight, 1188877338, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 28, 197, 26, 212, 220, 70], OperandSize::Qword)
}

