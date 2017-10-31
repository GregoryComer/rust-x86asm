use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vblendvpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM1)), operand4: Some(Direct(XMM4)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 65, 75, 201, 64], OperandSize::Dword)
}

#[test]
fn vblendvpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(ECX, ESI, Four, Some(OperandSize::Xmmword), None)), operand4: Some(Direct(XMM5)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 65, 75, 20, 177, 80], OperandSize::Dword)
}

#[test]
fn vblendvpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: Some(Direct(XMM0)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 65, 75, 224, 0], OperandSize::Qword)
}

#[test]
fn vblendvpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: Some(Direct(XMM6)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 65, 75, 50, 96], OperandSize::Qword)
}

#[test]
fn vblendvpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM1)), operand4: Some(Direct(YMM5)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 93, 75, 201, 80], OperandSize::Dword)
}

#[test]
fn vblendvpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 1365651260, Some(OperandSize::Ymmword), None)), operand4: Some(Direct(YMM7)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 75, 28, 85, 60, 47, 102, 81, 112], OperandSize::Dword)
}

#[test]
fn vblendvpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM3)), operand4: Some(Direct(YMM1)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 69, 75, 195, 16], OperandSize::Qword)
}

#[test]
fn vblendvpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(RCX, RCX, Two, Some(OperandSize::Ymmword), None)), operand4: Some(Direct(YMM0)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 75, 60, 73, 0], OperandSize::Qword)
}

