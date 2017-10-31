use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmuldq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 40, 200], OperandSize::Dword)
}

#[test]
fn pmuldq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULDQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(EDI, 1659796394, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 40, 191, 170, 123, 238, 98], OperandSize::Dword)
}

#[test]
fn pmuldq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 40, 206], OperandSize::Qword)
}

#[test]
fn pmuldq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULDQ, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 40, 46], OperandSize::Qword)
}

