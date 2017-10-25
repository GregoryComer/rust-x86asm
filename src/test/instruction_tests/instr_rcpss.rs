use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn rcpss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::RCPSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 83, 198], OperandSize::Dword)
}

#[test]
fn rcpss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::RCPSS, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 83, 6], OperandSize::Dword)
}

#[test]
fn rcpss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::RCPSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 83, 193], OperandSize::Qword)
}

#[test]
fn rcpss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::RCPSS, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 83, 6], OperandSize::Qword)
}

