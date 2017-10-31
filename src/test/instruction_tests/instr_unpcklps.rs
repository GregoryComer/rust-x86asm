use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn unpcklps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKLPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 20, 202], OperandSize::Dword)
}

#[test]
fn unpcklps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKLPS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 2002714870, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 20, 60, 69, 246, 0, 95, 119], OperandSize::Dword)
}

#[test]
fn unpcklps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKLPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 20, 237], OperandSize::Qword)
}

#[test]
fn unpcklps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKLPS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 1952101518, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 20, 36, 245, 142, 180, 90, 116], OperandSize::Qword)
}

