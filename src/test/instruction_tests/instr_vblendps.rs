use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vblendps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM5)), operand4: Some(Literal8(94)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 73, 12, 253, 94], OperandSize::Dword)
}

#[test]
fn vblendps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Two, 577840228, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(65)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 89, 12, 140, 75, 100, 36, 113, 34, 65], OperandSize::Dword)
}

#[test]
fn vblendps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM5)), operand4: Some(Literal8(1)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 12, 229, 1], OperandSize::Qword)
}

#[test]
fn vblendps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(RBX, 2021686624, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(9)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 65, 12, 139, 96, 125, 128, 120, 9], OperandSize::Qword)
}

#[test]
fn vblendps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM6)), operand4: Some(Literal8(55)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 117, 12, 198, 55], OperandSize::Dword)
}

#[test]
fn vblendps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(25)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 85, 12, 47, 25], OperandSize::Dword)
}

#[test]
fn vblendps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM0)), operand4: Some(Literal8(28)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 77, 12, 208, 28], OperandSize::Qword)
}

#[test]
fn vblendps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(RSI, RBX, Two, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(13)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 85, 12, 52, 94, 13], OperandSize::Qword)
}

