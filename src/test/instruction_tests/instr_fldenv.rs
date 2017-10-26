use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fldenv_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FLDENV, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 23516, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 163, 220, 91], OperandSize::Word)
}

#[test]
fn fldenv_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FLDENV, operand1: Some(IndirectScaledIndexed(ECX, EAX, Four, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 36, 129], OperandSize::Dword)
}

#[test]
fn fldenv_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FLDENV, operand1: Some(IndirectScaledDisplaced(RDX, Eight, 2082555032, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 36, 213, 152, 68, 33, 124], OperandSize::Qword)
}

