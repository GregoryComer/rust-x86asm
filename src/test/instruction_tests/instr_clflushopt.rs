use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn clflushopt_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CLFLUSHOPT, operand1: Some(IndirectDisplaced(DI, 145, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 174, 189, 145, 0], OperandSize::Word)
}

#[test]
fn clflushopt_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CLFLUSHOPT, operand1: Some(IndirectScaledIndexed(EAX, ECX, Eight, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 174, 60, 200], OperandSize::Dword)
}

#[test]
fn clflushopt_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CLFLUSHOPT, operand1: Some(IndirectScaledDisplaced(RCX, Two, 209791291, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 174, 60, 77, 59, 41, 129, 12], OperandSize::Qword)
}

