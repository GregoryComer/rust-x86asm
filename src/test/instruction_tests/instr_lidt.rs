use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn lidt_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LIDT, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 29125, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 152, 197, 113], OperandSize::Word)
}

#[test]
fn lidt_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LIDT, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Four, 924348613, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 156, 191, 197, 112, 24, 55], OperandSize::Dword)
}

#[test]
fn lidt_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LIDT, operand1: Some(IndirectDisplaced(RCX, 431027620, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 153, 164, 245, 176, 25], OperandSize::Qword)
}

