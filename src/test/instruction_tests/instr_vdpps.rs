use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vdpps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VDPPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(31)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 89, 64, 250, 31], OperandSize::Dword)
}

#[test]
fn vdpps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VDPPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(55)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 65, 64, 46, 55], OperandSize::Dword)
}

#[test]
fn vdpps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VDPPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM0)), operand4: Some(Literal8(74)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 73, 64, 208, 74], OperandSize::Qword)
}

#[test]
fn vdpps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VDPPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(36)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 65, 64, 35, 36], OperandSize::Qword)
}

#[test]
fn vdpps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VDPPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM2)), operand4: Some(Literal8(89)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 69, 64, 210, 89], OperandSize::Dword)
}

#[test]
fn vdpps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VDPPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(28)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 109, 64, 23, 28], OperandSize::Dword)
}

#[test]
fn vdpps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VDPPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM1)), operand4: Some(Literal8(125)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 85, 64, 209, 125], OperandSize::Qword)
}

#[test]
fn vdpps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VDPPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(RAX, RDX, Eight, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(12)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 101, 64, 4, 208, 12], OperandSize::Qword)
}

