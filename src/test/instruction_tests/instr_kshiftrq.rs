use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn kshiftrq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KSHIFTRQ, operand1: Some(Direct(K1)), operand2: Some(Direct(K4)), operand3: Some(Literal8(8)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 249, 49, 204, 8], OperandSize::Dword)
}

#[test]
fn kshiftrq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KSHIFTRQ, operand1: Some(Direct(K6)), operand2: Some(Direct(K4)), operand3: Some(Literal8(94)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 249, 49, 244, 94], OperandSize::Qword)
}

