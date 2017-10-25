use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vblendpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(10)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 97, 13, 236, 10], OperandSize::Dword)
}

#[test]
fn vblendpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(EAX, 1718824550, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(1)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 13, 168, 102, 46, 115, 102, 1], OperandSize::Dword)
}

#[test]
fn vblendpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM6)), operand4: Some(Literal8(118)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 97, 13, 222, 118], OperandSize::Qword)
}

#[test]
fn vblendpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(RDI, 318650137, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(54)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 89, 13, 175, 25, 55, 254, 18, 54], OperandSize::Qword)
}

#[test]
fn vblendpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM6)), operand4: Some(Literal8(59)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 85, 13, 214, 59], OperandSize::Dword)
}

#[test]
fn vblendpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(EDX, ECX, Eight, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(27)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 69, 13, 60, 202, 27], OperandSize::Dword)
}

#[test]
fn vblendpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM2)), operand4: Some(Literal8(13)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 77, 13, 210, 13], OperandSize::Qword)
}

#[test]
fn vblendpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 1158770483, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(32)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 101, 13, 4, 149, 51, 111, 17, 69, 32], OperandSize::Qword)
}

