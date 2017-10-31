use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn kxnord_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KXNORD, operand1: Some(Direct(K6)), operand2: Some(Direct(K3)), operand3: Some(Direct(K5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 229, 70, 245], OperandSize::Dword)
}

#[test]
fn kxnord_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KXNORD, operand1: Some(Direct(K2)), operand2: Some(Direct(K2)), operand3: Some(Direct(K2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 237, 70, 210], OperandSize::Qword)
}

