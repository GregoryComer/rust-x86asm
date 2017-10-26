use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vblendpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM5)), operand4: Some(Literal8(78)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 73, 13, 245, 78], OperandSize::Dword)
}

#[test]
fn vblendpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(66)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 73, 13, 33, 66], OperandSize::Dword)
}

#[test]
fn vblendpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(53)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 65, 13, 226, 53], OperandSize::Qword)
}

#[test]
fn vblendpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(24)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 13, 34, 24], OperandSize::Qword)
}

#[test]
fn vblendpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM3)), operand4: Some(Literal8(79)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 109, 13, 211, 79], OperandSize::Dword)
}

#[test]
fn vblendpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(ECX, EDX, Four, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(79)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 101, 13, 60, 145, 79], OperandSize::Dword)
}

#[test]
fn vblendpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM6)), operand4: Some(Literal8(25)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 109, 13, 246, 25], OperandSize::Qword)
}

#[test]
fn vblendpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 1021939763, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(45)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 93, 13, 20, 245, 51, 144, 233, 60, 45], OperandSize::Qword)
}

