use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fldcw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FLDCW, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 75, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 106, 75], OperandSize::Word)
}

#[test]
fn fldcw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FLDCW, operand1: Some(Indirect(ESI, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 46], OperandSize::Dword)
}

#[test]
fn fldcw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FLDCW, operand1: Some(IndirectScaledDisplaced(RDX, Two, 1826348273, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 44, 85, 241, 220, 219, 108], OperandSize::Qword)
}

