use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fsave_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FSAVE, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 6428, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[155, 221, 177, 28, 25], OperandSize::Word)
}

#[test]
fn fsave_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FSAVE, operand1: Some(Indirect(EDI, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[155, 221, 55], OperandSize::Dword)
}

#[test]
fn fsave_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FSAVE, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Eight, 1801092583, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[155, 221, 180, 246, 231, 125, 90, 107], OperandSize::Qword)
}

