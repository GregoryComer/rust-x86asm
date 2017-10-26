use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn roundss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDSS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(86)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 10, 204, 86], OperandSize::Dword)
}

#[test]
fn roundss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDSS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(ECX, EAX, Two, Some(OperandSize::Dword), None)), operand3: Some(Literal8(91)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 10, 36, 65, 91], OperandSize::Dword)
}

#[test]
fn roundss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(63)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 10, 197, 63], OperandSize::Qword)
}

#[test]
fn roundss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDSS, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(12)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 10, 50, 12], OperandSize::Qword)
}

