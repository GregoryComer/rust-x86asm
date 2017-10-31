use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn clflushopt_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CLFLUSHOPT, operand1: Some(IndirectDisplaced(BP, 19970, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 174, 190, 2, 78], OperandSize::Word)
}

#[test]
fn clflushopt_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CLFLUSHOPT, operand1: Some(IndirectScaledDisplaced(EAX, Two, 1935808977, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 174, 60, 69, 209, 25, 98, 115], OperandSize::Dword)
}

#[test]
fn clflushopt_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CLFLUSHOPT, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Eight, 281098546, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 174, 188, 222, 50, 57, 193, 16], OperandSize::Qword)
}

