use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn enter_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ENTER, operand1: Some(Literal16(12552)), operand2: Some(Literal8(24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[200, 8, 49, 24], OperandSize::Word)
}

#[test]
fn enter_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ENTER, operand1: Some(Literal16(931)), operand2: Some(Literal8(14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[200, 163, 3, 14], OperandSize::Dword)
}

#[test]
fn enter_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ENTER, operand1: Some(Literal16(17550)), operand2: Some(Literal8(73)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[200, 142, 68, 73], OperandSize::Qword)
}

