use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn invlpg_1() {
    run_test(&Instruction { mnemonic: Mnemonic::INVLPG, operand1: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 56], OperandSize::Word)
}

#[test]
fn invlpg_2() {
    run_test(&Instruction { mnemonic: Mnemonic::INVLPG, operand1: Some(IndirectDisplaced(EDX, 1589290773, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 186, 21, 167, 186, 94], OperandSize::Dword)
}

#[test]
fn invlpg_3() {
    run_test(&Instruction { mnemonic: Mnemonic::INVLPG, operand1: Some(Indirect(RCX, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 57], OperandSize::Qword)
}

