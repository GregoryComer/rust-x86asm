use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vdpps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VDPPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM3)), operand4: Some(Literal8(27)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 97, 64, 251, 27], OperandSize::Dword)
}

#[test]
fn vdpps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VDPPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(EDX, 1125906929, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(70)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 81, 64, 162, 241, 249, 27, 67, 70], OperandSize::Dword)
}

#[test]
fn vdpps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VDPPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM3)), operand4: Some(Literal8(40)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 73, 64, 219, 40], OperandSize::Qword)
}

#[test]
fn vdpps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VDPPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(RDI, 1105652357, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(88)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 81, 64, 191, 133, 234, 230, 65, 88], OperandSize::Qword)
}

#[test]
fn vdpps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VDPPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM5)), operand4: Some(Literal8(105)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 85, 64, 237, 105], OperandSize::Dword)
}

#[test]
fn vdpps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VDPPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(ECX, EAX, Eight, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(52)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 64, 36, 193, 52], OperandSize::Dword)
}

#[test]
fn vdpps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VDPPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM7)), operand4: Some(Literal8(102)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 85, 64, 231, 102], OperandSize::Qword)
}

#[test]
fn vdpps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VDPPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Two, 587983848, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(14)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 101, 64, 188, 90, 232, 235, 11, 35, 14], OperandSize::Qword)
}

