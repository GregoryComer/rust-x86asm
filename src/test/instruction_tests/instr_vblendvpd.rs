use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vblendvpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM7)), operand4: Some(Direct(XMM0)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 75, 255, 0], OperandSize::Dword)
}

#[test]
fn vblendvpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(EDX, 314445239, Some(OperandSize::Xmmword), None)), operand4: Some(Direct(XMM0)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 89, 75, 154, 183, 13, 190, 18, 0], OperandSize::Dword)
}

#[test]
fn vblendvpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM5)), operand4: Some(Direct(XMM3)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 97, 75, 237, 48], OperandSize::Qword)
}

#[test]
fn vblendvpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Four, 1122138105, Some(OperandSize::Xmmword), None)), operand4: Some(Direct(XMM6)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 75, 164, 152, 249, 119, 226, 66, 96], OperandSize::Qword)
}

#[test]
fn vblendvpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM6)), operand4: Some(Direct(YMM6)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 101, 75, 230, 96], OperandSize::Dword)
}

#[test]
fn vblendvpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Four, 1131721686, Some(OperandSize::Ymmword), None)), operand4: Some(Direct(YMM7)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 101, 75, 132, 137, 214, 179, 116, 67, 112], OperandSize::Dword)
}

#[test]
fn vblendvpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM4)), operand4: Some(Direct(YMM6)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 85, 75, 196, 96], OperandSize::Qword)
}

#[test]
fn vblendvpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(RCX, RBX, Four, Some(OperandSize::Ymmword), None)), operand4: Some(Direct(YMM7)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 85, 75, 44, 153, 112], OperandSize::Qword)
}

