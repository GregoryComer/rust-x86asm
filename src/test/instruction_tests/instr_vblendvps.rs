use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vblendvps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM1)), operand4: Some(Direct(XMM2)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 89, 74, 249, 32], OperandSize::Dword)
}

#[test]
fn vblendvps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(EAX, ECX, Four, Some(OperandSize::Xmmword), None)), operand4: Some(Direct(XMM7)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 74, 28, 136, 112], OperandSize::Dword)
}

#[test]
fn vblendvps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: Some(Direct(XMM2)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 74, 194, 32], OperandSize::Qword)
}

#[test]
fn vblendvps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(RDX, RDX, Two, Some(OperandSize::Xmmword), None)), operand4: Some(Direct(XMM0)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 74, 4, 82, 0], OperandSize::Qword)
}

#[test]
fn vblendvps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM5)), operand4: Some(Direct(YMM7)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 74, 245, 112], OperandSize::Dword)
}

#[test]
fn vblendvps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Two, 1823257297, Some(OperandSize::Ymmword), None)), operand4: Some(Direct(YMM5)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 85, 74, 164, 64, 209, 178, 172, 108, 80], OperandSize::Dword)
}

#[test]
fn vblendvps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM6)), operand4: Some(Direct(YMM5)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 69, 74, 206, 80], OperandSize::Qword)
}

#[test]
fn vblendvps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand4: Some(Direct(YMM2)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 69, 74, 57, 32], OperandSize::Qword)
}

