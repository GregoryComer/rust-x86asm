use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fsave_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FSAVE, operand1: Some(IndirectDisplaced(SI, 14159, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[155, 221, 180, 79, 55], OperandSize::Word)
}

#[test]
fn fsave_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FSAVE, operand1: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Eight, 486595323, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[155, 221, 180, 198, 251, 218, 0, 29], OperandSize::Dword)
}

#[test]
fn fsave_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FSAVE, operand1: Some(Indirect(RBX, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[155, 221, 51], OperandSize::Qword)
}

