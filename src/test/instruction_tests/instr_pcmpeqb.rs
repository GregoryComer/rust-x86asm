use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pcmpeqb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQB, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 116, 234], OperandSize::Dword)
}

#[test]
fn pcmpeqb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQB, operand1: Some(Direct(MM0)), operand2: Some(IndirectScaledDisplaced(EDX, Eight, 1396028528, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 116, 4, 213, 112, 180, 53, 83], OperandSize::Dword)
}

#[test]
fn pcmpeqb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQB, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 116, 222], OperandSize::Qword)
}

#[test]
fn pcmpeqb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQB, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Four, 1530167701, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 116, 188, 183, 149, 129, 52, 91], OperandSize::Qword)
}

#[test]
fn pcmpeqb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 116, 209], OperandSize::Dword)
}

#[test]
fn pcmpeqb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQB, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 116, 32], OperandSize::Dword)
}

#[test]
fn pcmpeqb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 116, 249], OperandSize::Qword)
}

#[test]
fn pcmpeqb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQB, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 116, 6], OperandSize::Qword)
}

