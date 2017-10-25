use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn ret_1() {
    run_test(&Instruction { mnemonic: Mnemonic::RET, operand1: Some(Literal16(21854)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[194, 94, 85], OperandSize::Word)
}

#[test]
fn ret_2() {
    run_test(&Instruction { mnemonic: Mnemonic::RET, operand1: Some(Literal16(18908)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[194, 220, 73], OperandSize::Dword)
}

#[test]
fn ret_3() {
    run_test(&Instruction { mnemonic: Mnemonic::RET, operand1: Some(Literal16(28935)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[194, 7, 113], OperandSize::Qword)
}

#[test]
fn ret_4() {
    run_test(&Instruction { mnemonic: Mnemonic::RET, operand1: Some(Literal16(6440)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[194, 40, 25], OperandSize::Word)
}

#[test]
fn ret_5() {
    run_test(&Instruction { mnemonic: Mnemonic::RET, operand1: Some(Literal16(25441)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[194, 97, 99], OperandSize::Dword)
}

#[test]
fn ret_6() {
    run_test(&Instruction { mnemonic: Mnemonic::RET, operand1: Some(Literal16(8297)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[194, 105, 32], OperandSize::Qword)
}

