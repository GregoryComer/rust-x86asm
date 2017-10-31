use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn ret_1() {
    run_test(&Instruction { mnemonic: Mnemonic::RET, operand1: Some(Literal16(10525)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[194, 29, 41], OperandSize::Word)
}

#[test]
fn ret_2() {
    run_test(&Instruction { mnemonic: Mnemonic::RET, operand1: Some(Literal16(3474)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[194, 146, 13], OperandSize::Dword)
}

#[test]
fn ret_3() {
    run_test(&Instruction { mnemonic: Mnemonic::RET, operand1: Some(Literal16(25026)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[194, 194, 97], OperandSize::Qword)
}

#[test]
fn ret_4() {
    run_test(&Instruction { mnemonic: Mnemonic::RET, operand1: Some(Literal16(15695)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[194, 79, 61], OperandSize::Word)
}

#[test]
fn ret_5() {
    run_test(&Instruction { mnemonic: Mnemonic::RET, operand1: Some(Literal16(6707)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[194, 51, 26], OperandSize::Dword)
}

#[test]
fn ret_6() {
    run_test(&Instruction { mnemonic: Mnemonic::RET, operand1: Some(Literal16(1551)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[194, 15, 6], OperandSize::Qword)
}

