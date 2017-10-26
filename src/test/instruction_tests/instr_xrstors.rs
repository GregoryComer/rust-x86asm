use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn xrstors_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XRSTORS, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 8, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 199, 88, 8], OperandSize::Word)
}

#[test]
fn xrstors_2() {
    run_test(&Instruction { mnemonic: Mnemonic::XRSTORS, operand1: Some(IndirectScaledIndexed(EDI, EDI, Two, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 199, 28, 127], OperandSize::Dword)
}

#[test]
fn xrstors_3() {
    run_test(&Instruction { mnemonic: Mnemonic::XRSTORS, operand1: Some(IndirectDisplaced(RSI, 1537805347, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 199, 158, 35, 12, 169, 91], OperandSize::Qword)
}

