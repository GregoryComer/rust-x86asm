use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn knotq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KNOTQ, operand1: Some(Direct(K5)), operand2: Some(Direct(K5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 248, 68, 237], OperandSize::Dword)
}

#[test]
fn knotq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KNOTQ, operand1: Some(Direct(K5)), operand2: Some(Direct(K5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 248, 68, 237], OperandSize::Qword)
}

