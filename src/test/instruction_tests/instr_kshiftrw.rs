use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn kshiftrw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KSHIFTRW, operand1: Some(Direct(K2)), operand2: Some(Direct(K7)), operand3: Some(Literal8(43)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 249, 48, 215, 43], OperandSize::Dword)
}

#[test]
fn kshiftrw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KSHIFTRW, operand1: Some(Direct(K2)), operand2: Some(Direct(K6)), operand3: Some(Literal8(104)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 249, 48, 214, 104], OperandSize::Qword)
}

