use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn lgdt_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LGDT, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 201, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 144, 201, 0], OperandSize::Word)
}

#[test]
fn lgdt_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LGDT, operand1: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Two, 772190312, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 148, 78, 104, 176, 6, 46], OperandSize::Dword)
}

#[test]
fn lgdt_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LGDT, operand1: Some(IndirectDisplaced(RAX, 439712380, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 144, 124, 122, 53, 26], OperandSize::Qword)
}

