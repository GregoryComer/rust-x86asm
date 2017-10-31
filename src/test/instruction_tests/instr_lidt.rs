use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn lidt_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LIDT, operand1: Some(IndirectDisplaced(DI, 212, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 157, 212, 0], OperandSize::Word)
}

#[test]
fn lidt_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LIDT, operand1: Some(IndirectScaledDisplaced(ESI, Eight, 592447778, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 28, 245, 34, 9, 80, 35], OperandSize::Dword)
}

#[test]
fn lidt_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LIDT, operand1: Some(IndirectScaledIndexed(RCX, RDX, Two, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 28, 81], OperandSize::Qword)
}

