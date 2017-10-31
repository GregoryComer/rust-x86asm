use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pcmpeqq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 41, 247], OperandSize::Dword)
}

#[test]
fn pcmpeqq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQQ, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Four, 1727789577, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 41, 132, 179, 9, 250, 251, 102], OperandSize::Dword)
}

#[test]
fn pcmpeqq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 41, 249], OperandSize::Qword)
}

#[test]
fn pcmpeqq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(RCX, 813791971, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 41, 185, 227, 122, 129, 48], OperandSize::Qword)
}

