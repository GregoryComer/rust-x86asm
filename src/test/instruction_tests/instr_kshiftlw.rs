use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn kshiftlw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KSHIFTLW, operand1: Some(Direct(K7)), operand2: Some(Direct(K3)), operand3: Some(Literal8(7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 249, 50, 251, 7], OperandSize::Dword)
}

#[test]
fn kshiftlw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KSHIFTLW, operand1: Some(Direct(K4)), operand2: Some(Direct(K6)), operand3: Some(Literal8(99)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 249, 50, 230, 99], OperandSize::Qword)
}

