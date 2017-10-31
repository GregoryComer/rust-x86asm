use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fnsave_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSAVE, operand1: Some(IndirectDisplaced(DI, 22125, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 181, 109, 86], OperandSize::Word)
}

#[test]
fn fnsave_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSAVE, operand1: Some(IndirectScaledDisplaced(ESI, Eight, 207534219, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 52, 245, 139, 184, 94, 12], OperandSize::Dword)
}

#[test]
fn fnsave_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSAVE, operand1: Some(IndirectScaledIndexed(RAX, RDX, Eight, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 52, 208], OperandSize::Qword)
}

