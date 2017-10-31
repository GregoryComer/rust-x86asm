use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn enter_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ENTER, operand1: Some(Literal16(1607)), operand2: Some(Literal8(20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[200, 71, 6, 20], OperandSize::Word)
}

#[test]
fn enter_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ENTER, operand1: Some(Literal16(26478)), operand2: Some(Literal8(123)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[200, 110, 103, 123], OperandSize::Dword)
}

#[test]
fn enter_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ENTER, operand1: Some(Literal16(6760)), operand2: Some(Literal8(88)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[200, 104, 26, 88], OperandSize::Qword)
}

