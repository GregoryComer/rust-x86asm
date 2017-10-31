use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn roundpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(62)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 9, 234, 62], OperandSize::Dword)
}

#[test]
fn roundpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDPD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(ECX, ECX, Two, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(44)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 9, 4, 73, 44], OperandSize::Dword)
}

#[test]
fn roundpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 9, 232, 2], OperandSize::Qword)
}

#[test]
fn roundpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDPD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 1594162194, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(99)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 9, 44, 205, 18, 252, 4, 95, 99], OperandSize::Qword)
}

