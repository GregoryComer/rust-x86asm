use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vblendvps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM1)), operand4: Some(Direct(XMM6)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 74, 193, 96], OperandSize::Dword)
}

#[test]
fn vblendvps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(ECX, 1684050701, Some(OperandSize::Xmmword), None)), operand4: Some(Direct(XMM1)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 73, 74, 177, 13, 147, 96, 100, 16], OperandSize::Dword)
}

#[test]
fn vblendvps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: Some(Direct(XMM7)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 65, 74, 210, 112], OperandSize::Qword)
}

#[test]
fn vblendvps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(RSI, 474977750, Some(OperandSize::Xmmword), None)), operand4: Some(Direct(XMM3)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 74, 142, 214, 149, 79, 28, 48], OperandSize::Qword)
}

#[test]
fn vblendvps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM4)), operand4: Some(Direct(YMM0)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 74, 220, 0], OperandSize::Dword)
}

#[test]
fn vblendvps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(EAX, 48328267, Some(OperandSize::Ymmword), None)), operand4: Some(Direct(YMM5)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 85, 74, 144, 75, 110, 225, 2, 80], OperandSize::Dword)
}

#[test]
fn vblendvps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM7)), operand4: Some(Direct(YMM1)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 117, 74, 239, 16], OperandSize::Qword)
}

#[test]
fn vblendvps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Two, 800305029, Some(OperandSize::Ymmword), None)), operand4: Some(Direct(YMM3)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 69, 74, 132, 72, 133, 175, 179, 47, 48], OperandSize::Qword)
}

