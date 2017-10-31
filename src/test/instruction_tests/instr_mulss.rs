use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn mulss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MULSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 89, 252], OperandSize::Dword)
}

#[test]
fn mulss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MULSS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(EDX, EDX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 89, 12, 210], OperandSize::Dword)
}

#[test]
fn mulss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MULSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 89, 199], OperandSize::Qword)
}

#[test]
fn mulss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MULSS, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 89, 32], OperandSize::Qword)
}

