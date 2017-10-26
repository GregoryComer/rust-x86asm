use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fsave_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FSAVE, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 121, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[155, 221, 113, 121], OperandSize::Word)
}

#[test]
fn fsave_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FSAVE, operand1: Some(IndirectScaledIndexed(ECX, EDX, Eight, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[155, 221, 52, 209], OperandSize::Dword)
}

#[test]
fn fsave_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FSAVE, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Two, 1940199638, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[155, 221, 180, 65, 214, 24, 165, 115], OperandSize::Qword)
}

