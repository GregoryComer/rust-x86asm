use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovmskps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVMSKPS, operand1: Some(Direct(EDI)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 80, 248], OperandSize::Dword)
}

#[test]
fn vmovmskps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVMSKPS, operand1: Some(Direct(RBX)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 80, 219], OperandSize::Qword)
}

#[test]
fn vmovmskps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVMSKPS, operand1: Some(Direct(EDX)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 80, 211], OperandSize::Dword)
}

#[test]
fn vmovmskps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVMSKPS, operand1: Some(Direct(RBX)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 80, 219], OperandSize::Qword)
}

