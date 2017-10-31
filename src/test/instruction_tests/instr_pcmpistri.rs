use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pcmpistri_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPISTRI, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(108)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 99, 226, 108], OperandSize::Dword)
}

#[test]
fn pcmpistri_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPISTRI, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(ESI, 1567498036, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(77)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 99, 150, 52, 31, 110, 93, 77], OperandSize::Dword)
}

#[test]
fn pcmpistri_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPISTRI, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(32)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 99, 234, 32], OperandSize::Qword)
}

#[test]
fn pcmpistri_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPISTRI, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(RSI, Four, 1637534634, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(98)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 99, 60, 181, 170, 203, 154, 97, 98], OperandSize::Qword)
}

