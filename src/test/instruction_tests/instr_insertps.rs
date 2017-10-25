use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn insertps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::INSERTPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(76)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 33, 206, 76], OperandSize::Dword)
}

#[test]
fn insertps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::INSERTPS, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand3: Some(Literal8(118)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 33, 46, 118], OperandSize::Dword)
}

#[test]
fn insertps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::INSERTPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(69)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 33, 211, 69], OperandSize::Qword)
}

#[test]
fn insertps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::INSERTPS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(RSI, 321271997, Some(OperandSize::Dword), None)), operand3: Some(Literal8(34)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 33, 134, 189, 56, 38, 19, 34], OperandSize::Qword)
}

