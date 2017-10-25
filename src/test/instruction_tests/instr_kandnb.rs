use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn kandnb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KANDNB, operand1: Some(Direct(K5)), operand2: Some(Direct(K3)), operand3: Some(Direct(K4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 66, 236], OperandSize::Dword)
}

#[test]
fn kandnb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KANDNB, operand1: Some(Direct(K4)), operand2: Some(Direct(K6)), operand3: Some(Direct(K3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 66, 227], OperandSize::Qword)
}

