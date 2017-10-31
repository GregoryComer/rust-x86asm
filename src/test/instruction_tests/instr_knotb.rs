use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn knotb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KNOTB, operand1: Some(Direct(K3)), operand2: Some(Direct(K3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 68, 219], OperandSize::Dword)
}

#[test]
fn knotb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KNOTB, operand1: Some(Direct(K1)), operand2: Some(Direct(K1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 68, 201], OperandSize::Qword)
}

