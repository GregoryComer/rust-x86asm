use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pcmpeqq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 41, 219], OperandSize::Dword)
}

#[test]
fn pcmpeqq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQQ, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 503835536, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 41, 20, 133, 144, 235, 7, 30], OperandSize::Dword)
}

#[test]
fn pcmpeqq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 41, 237], OperandSize::Qword)
}

#[test]
fn pcmpeqq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQQ, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 41, 22], OperandSize::Qword)
}

