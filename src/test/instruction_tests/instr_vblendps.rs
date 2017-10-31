use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vblendps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM3)), operand4: Some(Literal8(8)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 97, 12, 195, 8], OperandSize::Dword)
}

#[test]
fn vblendps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(3)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 65, 12, 9, 3], OperandSize::Dword)
}

#[test]
fn vblendps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(64)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 65, 12, 236, 64], OperandSize::Qword)
}

#[test]
fn vblendps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(RSI, RSI, Four, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(119)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 89, 12, 44, 182, 119], OperandSize::Qword)
}

#[test]
fn vblendps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM4)), operand4: Some(Literal8(115)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 77, 12, 212, 115], OperandSize::Dword)
}

#[test]
fn vblendps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(117)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 109, 12, 41, 117], OperandSize::Dword)
}

#[test]
fn vblendps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM3)), operand4: Some(Literal8(96)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 93, 12, 235, 96], OperandSize::Qword)
}

#[test]
fn vblendps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(RDX, 1870444861, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(32)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 93, 12, 146, 61, 185, 124, 111, 32], OperandSize::Qword)
}

