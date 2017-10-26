use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn lgdt_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LGDT, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 7120, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 145, 208, 27], OperandSize::Word)
}

#[test]
fn lgdt_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LGDT, operand1: Some(IndirectScaledIndexed(EBX, EDI, Eight, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 20, 251], OperandSize::Dword)
}

#[test]
fn lgdt_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LGDT, operand1: Some(IndirectScaledIndexed(RCX, RSI, Two, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 20, 113], OperandSize::Qword)
}

