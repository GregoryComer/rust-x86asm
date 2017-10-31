use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vblendvps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM6)), operand4: Some(Direct(XMM2)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 81, 74, 230, 32], OperandSize::Dword)
}

#[test]
fn vblendvps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Eight, 1390178966, Some(OperandSize::Xmmword), None)), operand4: Some(Direct(XMM6)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 81, 74, 132, 195, 150, 114, 220, 82, 96], OperandSize::Dword)
}

#[test]
fn vblendvps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM7)), operand4: Some(Direct(XMM4)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 73, 74, 239, 64], OperandSize::Qword)
}

#[test]
fn vblendvps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(RAX, RAX, Eight, Some(OperandSize::Xmmword), None)), operand4: Some(Direct(XMM2)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 105, 74, 28, 192, 32], OperandSize::Qword)
}

#[test]
fn vblendvps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM4)), operand4: Some(Direct(YMM3)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 117, 74, 252, 48], OperandSize::Dword)
}

#[test]
fn vblendvps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(ECX, 1124332539, Some(OperandSize::Ymmword), None)), operand4: Some(Direct(YMM4)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 85, 74, 177, 251, 243, 3, 67, 64], OperandSize::Dword)
}

#[test]
fn vblendvps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM3)), operand4: Some(Direct(YMM4)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 93, 74, 195, 64], OperandSize::Qword)
}

#[test]
fn vblendvps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(RBX, RDX, Two, Some(OperandSize::Ymmword), None)), operand4: Some(Direct(YMM3)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 85, 74, 36, 83, 48], OperandSize::Qword)
}

