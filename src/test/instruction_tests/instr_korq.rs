use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn korq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KORQ, operand1: Some(Direct(K1)), operand2: Some(Direct(K7)), operand3: Some(Direct(K3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 196, 69, 203], OperandSize::Dword)
}

#[test]
fn korq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KORQ, operand1: Some(Direct(K1)), operand2: Some(Direct(K5)), operand3: Some(Direct(K3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 212, 69, 203], OperandSize::Qword)
}

