use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn kxnorw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KXNORW, operand1: Some(Direct(K3)), operand2: Some(Direct(K2)), operand3: Some(Direct(K2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 236, 70, 218], OperandSize::Dword)
}

#[test]
fn kxnorw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KXNORW, operand1: Some(Direct(K3)), operand2: Some(Direct(K1)), operand3: Some(Direct(K5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 244, 70, 221], OperandSize::Qword)
}

