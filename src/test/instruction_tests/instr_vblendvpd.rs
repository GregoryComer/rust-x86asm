use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vblendvpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: Some(Direct(XMM0)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 89, 75, 205, 0], OperandSize::Dword)
}

#[test]
fn vblendvpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(EAX, EAX, Two, Some(OperandSize::Xmmword), None)), operand4: Some(Direct(XMM4)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 105, 75, 20, 64, 64], OperandSize::Dword)
}

#[test]
fn vblendvpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM7)), operand4: Some(Direct(XMM1)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 97, 75, 231, 16], OperandSize::Qword)
}

#[test]
fn vblendvpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(RCX, 2047278942, Some(OperandSize::Xmmword), None)), operand4: Some(Direct(XMM6)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 105, 75, 177, 94, 255, 6, 122, 96], OperandSize::Qword)
}

#[test]
fn vblendvpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM3)), operand4: Some(Direct(YMM7)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 93, 75, 203, 112], OperandSize::Dword)
}

#[test]
fn vblendvpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(EDX, 360630156, Some(OperandSize::Ymmword), None)), operand4: Some(Direct(YMM4)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 101, 75, 138, 140, 199, 126, 21, 64], OperandSize::Dword)
}

#[test]
fn vblendvpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM4)), operand4: Some(Direct(YMM0)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 85, 75, 196, 0], OperandSize::Qword)
}

#[test]
fn vblendvpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 2010333767, Some(OperandSize::Ymmword), None)), operand4: Some(Direct(YMM1)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 117, 75, 44, 253, 71, 66, 211, 119, 16], OperandSize::Qword)
}

