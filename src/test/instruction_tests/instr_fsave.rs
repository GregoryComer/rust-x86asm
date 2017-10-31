use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fsave_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FSAVE, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 1, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[155, 221, 113, 1], OperandSize::Word)
}

#[test]
fn fsave_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FSAVE, operand1: Some(IndirectScaledIndexed(ESI, EBX, Eight, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[155, 221, 52, 222], OperandSize::Dword)
}

#[test]
fn fsave_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FSAVE, operand1: Some(IndirectDisplaced(RDX, 2113508643, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[155, 221, 178, 35, 149, 249, 125], OperandSize::Qword)
}

