use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn lidt_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LIDT, operand1: Some(IndirectDisplaced(SI, 169, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 156, 169, 0], OperandSize::Word)
}

#[test]
fn lidt_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LIDT, operand1: Some(Indirect(EAX, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 24], OperandSize::Dword)
}

#[test]
fn lidt_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LIDT, operand1: Some(Indirect(RCX, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 25], OperandSize::Qword)
}

