use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn ret_1() {
    run_test(&Instruction { mnemonic: Mnemonic::RET, operand1: Some(Literal16(21061)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[194, 69, 82], OperandSize::Word)
}

#[test]
fn ret_2() {
    run_test(&Instruction { mnemonic: Mnemonic::RET, operand1: Some(Literal16(13743)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[194, 175, 53], OperandSize::Dword)
}

#[test]
fn ret_3() {
    run_test(&Instruction { mnemonic: Mnemonic::RET, operand1: Some(Literal16(28970)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[194, 42, 113], OperandSize::Qword)
}

#[test]
fn ret_4() {
    run_test(&Instruction { mnemonic: Mnemonic::RET, operand1: Some(Literal16(638)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[194, 126, 2], OperandSize::Word)
}

#[test]
fn ret_5() {
    run_test(&Instruction { mnemonic: Mnemonic::RET, operand1: Some(Literal16(8137)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[194, 201, 31], OperandSize::Dword)
}

#[test]
fn ret_6() {
    run_test(&Instruction { mnemonic: Mnemonic::RET, operand1: Some(Literal16(7231)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[194, 63, 28], OperandSize::Qword)
}

