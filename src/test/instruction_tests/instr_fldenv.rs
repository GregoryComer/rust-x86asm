use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fldenv_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FLDENV, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 30023, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 162, 71, 117], OperandSize::Word)
}

#[test]
fn fldenv_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FLDENV, operand1: Some(IndirectDisplaced(ECX, 591000451, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 161, 131, 243, 57, 35], OperandSize::Dword)
}

#[test]
fn fldenv_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FLDENV, operand1: Some(IndirectScaledDisplaced(RCX, Two, 189866028, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 36, 77, 44, 32, 81, 11], OperandSize::Qword)
}

