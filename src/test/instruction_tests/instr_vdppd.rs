use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vdppd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VDPPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM1)), operand4: Some(Literal8(57)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 65, 65, 217, 57], OperandSize::Dword)
}

#[test]
fn vdppd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VDPPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 1848221464, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(91)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 89, 65, 28, 213, 24, 159, 41, 110, 91], OperandSize::Dword)
}

#[test]
fn vdppd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VDPPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: Some(Literal8(8)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 65, 235, 8], OperandSize::Qword)
}

#[test]
fn vdppd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VDPPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(RBX, RDX, Eight, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(127)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 65, 52, 211, 127], OperandSize::Qword)
}

