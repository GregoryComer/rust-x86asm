use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pcmpeqq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 41, 209], OperandSize::Dword)
}

#[test]
fn pcmpeqq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQQ, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 41, 26], OperandSize::Dword)
}

#[test]
fn pcmpeqq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 41, 209], OperandSize::Qword)
}

#[test]
fn pcmpeqq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQQ, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 779753408, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 41, 28, 205, 192, 23, 122, 46], OperandSize::Qword)
}

