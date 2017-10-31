use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn clflush_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CLFLUSH, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 25176, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 186, 88, 98], OperandSize::Word)
}

#[test]
fn clflush_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CLFLUSH, operand1: Some(IndirectScaledIndexed(EAX, EBX, Eight, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 60, 216], OperandSize::Dword)
}

#[test]
fn clflush_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CLFLUSH, operand1: Some(Indirect(RDX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 58], OperandSize::Qword)
}

