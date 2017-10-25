use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn kortestb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KORTESTB, operand1: Some(Direct(K5)), operand2: Some(Direct(K4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 152, 236], OperandSize::Dword)
}

#[test]
fn kortestb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KORTESTB, operand1: Some(Direct(K5)), operand2: Some(Direct(K7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 152, 239], OperandSize::Qword)
}

