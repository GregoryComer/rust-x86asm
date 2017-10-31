use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn kandnq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KANDNQ, operand1: Some(Direct(K7)), operand2: Some(Direct(K2)), operand3: Some(Direct(K7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 236, 66, 255], OperandSize::Dword)
}

#[test]
fn kandnq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KANDNQ, operand1: Some(Direct(K4)), operand2: Some(Direct(K5)), operand3: Some(Direct(K5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 212, 66, 229], OperandSize::Qword)
}

