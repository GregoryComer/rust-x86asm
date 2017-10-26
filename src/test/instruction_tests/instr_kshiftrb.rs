use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn kshiftrb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KSHIFTRB, operand1: Some(Direct(K6)), operand2: Some(Direct(K5)), operand3: Some(Literal8(36)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 48, 245, 36], OperandSize::Dword)
}

#[test]
fn kshiftrb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KSHIFTRB, operand1: Some(Direct(K7)), operand2: Some(Direct(K5)), operand3: Some(Literal8(1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 48, 253, 1], OperandSize::Qword)
}

