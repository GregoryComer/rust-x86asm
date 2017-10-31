use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn kshiftlq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KSHIFTLQ, operand1: Some(Direct(K5)), operand2: Some(Direct(K7)), operand3: Some(Literal8(105)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 249, 51, 239, 105], OperandSize::Dword)
}

#[test]
fn kshiftlq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KSHIFTLQ, operand1: Some(Direct(K3)), operand2: Some(Direct(K3)), operand3: Some(Literal8(40)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 249, 51, 219, 40], OperandSize::Qword)
}

