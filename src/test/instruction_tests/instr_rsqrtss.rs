use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn rsqrtss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::RSQRTSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 82, 213], OperandSize::Dword)
}

#[test]
fn rsqrtss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::RSQRTSS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(EDX, 763824, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 82, 178, 176, 167, 11, 0], OperandSize::Dword)
}

#[test]
fn rsqrtss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::RSQRTSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 82, 212], OperandSize::Qword)
}

#[test]
fn rsqrtss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::RSQRTSS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Four, 275238251, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 82, 132, 151, 107, 205, 103, 16], OperandSize::Qword)
}

