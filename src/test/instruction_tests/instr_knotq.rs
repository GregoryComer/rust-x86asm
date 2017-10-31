use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn knotq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KNOTQ, operand1: Some(Direct(K6)), operand2: Some(Direct(K7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 248, 68, 247], OperandSize::Dword)
}

#[test]
fn knotq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KNOTQ, operand1: Some(Direct(K1)), operand2: Some(Direct(K3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 248, 68, 203], OperandSize::Qword)
}

