use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn enter_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ENTER, operand1: Some(Literal16(11191)), operand2: Some(Literal8(80)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[200, 183, 43, 80], OperandSize::Word)
}

#[test]
fn enter_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ENTER, operand1: Some(Literal16(24567)), operand2: Some(Literal8(60)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[200, 247, 95, 60], OperandSize::Dword)
}

#[test]
fn enter_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ENTER, operand1: Some(Literal16(21777)), operand2: Some(Literal8(75)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[200, 17, 85, 75], OperandSize::Qword)
}

