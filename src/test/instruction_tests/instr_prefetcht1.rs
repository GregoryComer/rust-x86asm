use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn prefetcht1_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHT1, operand1: Some(Indirect(BX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 24, 23], OperandSize::Word)
}

#[test]
fn prefetcht1_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHT1, operand1: Some(Indirect(EDI, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 24, 23], OperandSize::Dword)
}

#[test]
fn prefetcht1_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHT1, operand1: Some(Indirect(RSI, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 24, 22], OperandSize::Qword)
}

