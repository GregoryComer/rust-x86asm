use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sqrtss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SQRTSS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 81, 203], OperandSize::Dword)
}

#[test]
fn sqrtss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SQRTSS, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 81, 0], OperandSize::Dword)
}

#[test]
fn sqrtss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SQRTSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 81, 249], OperandSize::Qword)
}

#[test]
fn sqrtss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SQRTSS, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 81, 59], OperandSize::Qword)
}

