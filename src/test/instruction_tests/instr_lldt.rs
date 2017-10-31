use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn lldt_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LLDT, operand1: Some(Direct(SP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 212], OperandSize::Word)
}

#[test]
fn lldt_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LLDT, operand1: Some(Memory(13110, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 22, 54, 51], OperandSize::Word)
}

#[test]
fn lldt_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LLDT, operand1: Some(Direct(SP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 212], OperandSize::Dword)
}

#[test]
fn lldt_4() {
    run_test(&Instruction { mnemonic: Mnemonic::LLDT, operand1: Some(IndirectDisplaced(ECX, 970763116, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 145, 108, 171, 220, 57], OperandSize::Dword)
}

#[test]
fn lldt_5() {
    run_test(&Instruction { mnemonic: Mnemonic::LLDT, operand1: Some(Direct(SI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 214], OperandSize::Qword)
}

#[test]
fn lldt_6() {
    run_test(&Instruction { mnemonic: Mnemonic::LLDT, operand1: Some(IndirectScaledDisplaced(RDI, Four, 1590599123, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 20, 189, 211, 157, 206, 94], OperandSize::Qword)
}

