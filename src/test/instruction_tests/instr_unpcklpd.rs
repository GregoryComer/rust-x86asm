use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn unpcklpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKLPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 20, 230], OperandSize::Dword)
}

#[test]
fn unpcklpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKLPD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Two, 633098928, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 20, 180, 81, 176, 82, 188, 37], OperandSize::Dword)
}

#[test]
fn unpcklpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKLPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 20, 202], OperandSize::Qword)
}

#[test]
fn unpcklpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKLPD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(RSI, RAX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 20, 28, 70], OperandSize::Qword)
}

