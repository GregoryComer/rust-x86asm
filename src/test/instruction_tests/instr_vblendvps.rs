use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vblendvps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: Some(Direct(XMM0)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 65, 74, 211, 0], OperandSize::Dword)
}

#[test]
fn vblendvps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 738491097, Some(OperandSize::Xmmword), None)), operand4: Some(Direct(XMM5)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 89, 74, 28, 205, 217, 122, 4, 44, 80], OperandSize::Dword)
}

#[test]
fn vblendvps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM7)), operand4: Some(Direct(XMM3)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 81, 74, 231, 48], OperandSize::Qword)
}

#[test]
fn vblendvps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(RDX, 1240869001, Some(OperandSize::Xmmword), None)), operand4: Some(Direct(XMM7)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 73, 74, 146, 137, 40, 246, 73, 112], OperandSize::Qword)
}

#[test]
fn vblendvps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM6)), operand4: Some(Direct(YMM3)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 77, 74, 238, 48], OperandSize::Dword)
}

#[test]
fn vblendvps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 763893613, Some(OperandSize::Ymmword), None)), operand4: Some(Direct(YMM2)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 77, 74, 4, 221, 109, 23, 136, 45, 32], OperandSize::Dword)
}

#[test]
fn vblendvps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM4)), operand4: Some(Direct(YMM6)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 93, 74, 220, 96], OperandSize::Qword)
}

#[test]
fn vblendvps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 79068427, Some(OperandSize::Ymmword), None)), operand4: Some(Direct(YMM4)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 85, 74, 12, 93, 11, 125, 182, 4, 64], OperandSize::Qword)
}

