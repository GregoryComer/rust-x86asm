use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn prefetcht2_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHT2, operand1: Some(IndirectDisplaced(BP, 229, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 24, 158, 229, 0], OperandSize::Word)
}

#[test]
fn prefetcht2_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHT2, operand1: Some(Indirect(EAX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 24, 24], OperandSize::Dword)
}

#[test]
fn prefetcht2_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHT2, operand1: Some(IndirectScaledDisplaced(RDX, Eight, 1710920486, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 24, 28, 213, 38, 147, 250, 101], OperandSize::Qword)
}

