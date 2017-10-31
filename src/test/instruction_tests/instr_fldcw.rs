use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fldcw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FLDCW, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 12882, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 168, 82, 50], OperandSize::Word)
}

#[test]
fn fldcw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FLDCW, operand1: Some(IndirectScaledDisplaced(ESI, Eight, 1834017883, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 44, 245, 91, 228, 80, 109], OperandSize::Dword)
}

#[test]
fn fldcw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FLDCW, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Eight, 1026182504, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 172, 217, 104, 77, 42, 61], OperandSize::Qword)
}

