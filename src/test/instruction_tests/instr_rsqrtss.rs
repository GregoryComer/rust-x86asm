use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn rsqrtss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::RSQRTSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 82, 248], OperandSize::Dword)
}

#[test]
fn rsqrtss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::RSQRTSS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(EBX, 134589311, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 82, 187, 127, 171, 5, 8], OperandSize::Dword)
}

#[test]
fn rsqrtss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::RSQRTSS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 82, 220], OperandSize::Qword)
}

#[test]
fn rsqrtss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::RSQRTSS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(RCX, 1736486909, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 82, 137, 253, 175, 128, 103], OperandSize::Qword)
}

