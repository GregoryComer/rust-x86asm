use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn clflush_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CLFLUSH, operand1: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 56], OperandSize::Word)
}

#[test]
fn clflush_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CLFLUSH, operand1: Some(IndirectDisplaced(ECX, 1104857340, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 185, 252, 200, 218, 65], OperandSize::Dword)
}

#[test]
fn clflush_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CLFLUSH, operand1: Some(Indirect(RBX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 59], OperandSize::Qword)
}

