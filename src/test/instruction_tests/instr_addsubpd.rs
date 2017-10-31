use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn addsubpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSUBPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 208, 211], OperandSize::Dword)
}

#[test]
fn addsubpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSUBPD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(EDX, ECX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 208, 44, 202], OperandSize::Dword)
}

#[test]
fn addsubpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSUBPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 208, 198], OperandSize::Qword)
}

#[test]
fn addsubpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSUBPD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(RCX, RBX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 208, 20, 153], OperandSize::Qword)
}

