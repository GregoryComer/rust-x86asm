use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn kshiftlw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KSHIFTLW, operand1: Some(Direct(K7)), operand2: Some(Direct(K1)), operand3: Some(Literal8(22)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 249, 50, 249, 22], OperandSize::Dword)
}

#[test]
fn kshiftlw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KSHIFTLW, operand1: Some(Direct(K1)), operand2: Some(Direct(K5)), operand3: Some(Literal8(122)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 249, 50, 205, 122], OperandSize::Qword)
}

