use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn aesenclast_1() {
    run_test(&Instruction { mnemonic: Mnemonic::AESENCLAST, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 221, 201], OperandSize::Dword)
}

#[test]
fn aesenclast_2() {
    run_test(&Instruction { mnemonic: Mnemonic::AESENCLAST, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 221, 57], OperandSize::Dword)
}

#[test]
fn aesenclast_3() {
    run_test(&Instruction { mnemonic: Mnemonic::AESENCLAST, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 221, 199], OperandSize::Qword)
}

#[test]
fn aesenclast_4() {
    run_test(&Instruction { mnemonic: Mnemonic::AESENCLAST, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 221, 7], OperandSize::Qword)
}

