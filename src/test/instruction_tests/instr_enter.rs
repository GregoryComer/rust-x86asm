use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn enter_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ENTER, operand1: Some(Literal16(20343)), operand2: Some(Literal8(88)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[200, 119, 79, 88], OperandSize::Word)
}

#[test]
fn enter_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ENTER, operand1: Some(Literal16(13336)), operand2: Some(Literal8(52)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[200, 24, 52, 52], OperandSize::Dword)
}

#[test]
fn enter_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ENTER, operand1: Some(Literal16(5958)), operand2: Some(Literal8(26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[200, 70, 23, 26], OperandSize::Qword)
}

