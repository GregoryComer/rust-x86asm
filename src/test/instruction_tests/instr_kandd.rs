use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn kandd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KANDD, operand1: Some(Direct(K5)), operand2: Some(Direct(K4)), operand3: Some(Direct(K3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 221, 65, 235], OperandSize::Dword)
}

#[test]
fn kandd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KANDD, operand1: Some(Direct(K6)), operand2: Some(Direct(K1)), operand3: Some(Direct(K5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 245, 65, 245], OperandSize::Qword)
}

