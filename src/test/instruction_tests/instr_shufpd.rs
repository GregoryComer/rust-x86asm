use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn shufpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHUFPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(72)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 198, 205, 72], OperandSize::Dword)
}

#[test]
fn shufpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHUFPD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(ECX, ECX, Two, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(61)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 198, 36, 73, 61], OperandSize::Dword)
}

#[test]
fn shufpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHUFPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(93)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 198, 225, 93], OperandSize::Qword)
}

#[test]
fn shufpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHUFPD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 1695830654, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(28)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 198, 12, 245, 126, 82, 20, 101, 28], OperandSize::Qword)
}

