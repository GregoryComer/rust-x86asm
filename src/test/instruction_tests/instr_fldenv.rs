use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fldenv_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FLDENV, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 179, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 161, 179, 0], OperandSize::Word)
}

#[test]
fn fldenv_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FLDENV, operand1: Some(IndirectDisplaced(EAX, 1381912859, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 160, 27, 81, 94, 82], OperandSize::Dword)
}

#[test]
fn fldenv_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FLDENV, operand1: Some(Indirect(RAX, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 32], OperandSize::Qword)
}

