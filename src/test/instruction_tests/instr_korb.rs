use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn korb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KORB, operand1: Some(Direct(K4)), operand2: Some(Direct(K5)), operand3: Some(Direct(K6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 69, 230], OperandSize::Dword)
}

#[test]
fn korb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KORB, operand1: Some(Direct(K5)), operand2: Some(Direct(K3)), operand3: Some(Direct(K3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 69, 235], OperandSize::Qword)
}

