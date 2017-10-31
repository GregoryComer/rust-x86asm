use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn korb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KORB, operand1: Some(Direct(K2)), operand2: Some(Direct(K2)), operand3: Some(Direct(K6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 69, 214], OperandSize::Dword)
}

#[test]
fn korb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KORB, operand1: Some(Direct(K3)), operand2: Some(Direct(K1)), operand3: Some(Direct(K3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 69, 219], OperandSize::Qword)
}

