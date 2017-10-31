use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vdppd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VDPPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(44)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 65, 242, 44], OperandSize::Dword)
}

#[test]
fn vdppd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VDPPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 1604352793, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(42)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 65, 20, 77, 25, 123, 160, 95, 42], OperandSize::Dword)
}

#[test]
fn vdppd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VDPPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM7)), operand4: Some(Literal8(72)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 89, 65, 247, 72], OperandSize::Qword)
}

#[test]
fn vdppd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VDPPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(RDX, 978759917, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(13)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 73, 65, 162, 237, 176, 86, 58, 13], OperandSize::Qword)
}

