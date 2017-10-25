use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pcmpistrm_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPISTRM, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(116)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 98, 229, 116], OperandSize::Dword)
}

#[test]
fn pcmpistrm_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPISTRM, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(EDX, EBX, Two, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(113)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 98, 20, 90, 113], OperandSize::Dword)
}

#[test]
fn pcmpistrm_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPISTRM, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(112)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 98, 248, 112], OperandSize::Qword)
}

#[test]
fn pcmpistrm_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPISTRM, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Four, 2112367905, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(123)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 98, 148, 142, 33, 45, 232, 125, 123], OperandSize::Qword)
}

