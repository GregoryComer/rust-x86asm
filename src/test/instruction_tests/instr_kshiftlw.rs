use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn kshiftlw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KSHIFTLW, operand1: Some(Direct(K3)), operand2: Some(Direct(K5)), operand3: Some(Literal8(70)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 249, 50, 221, 70], OperandSize::Dword)
}

#[test]
fn kshiftlw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KSHIFTLW, operand1: Some(Direct(K1)), operand2: Some(Direct(K3)), operand3: Some(Literal8(23)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 249, 50, 203, 23], OperandSize::Qword)
}

