use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fstsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTSW, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 205, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[155, 221, 185, 205, 0], OperandSize::Word)
}

#[test]
fn fstsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTSW, operand1: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Two, 209835508, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[155, 221, 188, 86, 244, 213, 129, 12], OperandSize::Dword)
}

#[test]
fn fstsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTSW, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Eight, 812962454, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[155, 221, 188, 207, 150, 210, 116, 48], OperandSize::Qword)
}

