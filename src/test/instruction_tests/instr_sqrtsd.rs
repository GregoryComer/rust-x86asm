use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sqrtsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SQRTSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 81, 251], OperandSize::Dword)
}

#[test]
fn sqrtsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SQRTSD, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 81, 1], OperandSize::Dword)
}

#[test]
fn sqrtsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SQRTSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 81, 211], OperandSize::Qword)
}

#[test]
fn sqrtsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SQRTSD, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 81, 30], OperandSize::Qword)
}

