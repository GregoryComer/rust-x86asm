use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vroundss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM3)), operand4: Some(Literal8(68)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 105, 10, 235, 68], OperandSize::Dword)
}

#[test]
fn vroundss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Two, 31729548, Some(OperandSize::Dword), None)), operand4: Some(Literal8(61)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 10, 132, 80, 140, 39, 228, 1, 61], OperandSize::Dword)
}

#[test]
fn vroundss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM7)), operand4: Some(Literal8(123)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 10, 215, 123], OperandSize::Qword)
}

#[test]
fn vroundss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Two, 1840880868, Some(OperandSize::Dword), None)), operand4: Some(Literal8(95)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 97, 10, 148, 112, 228, 156, 185, 109, 95], OperandSize::Qword)
}

