use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vroundpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(76)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 9, 233, 76], OperandSize::Dword)
}

#[test]
fn vroundpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(EDI, 1121239572, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(127)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 9, 183, 20, 194, 212, 66, 127], OperandSize::Dword)
}

#[test]
fn vroundpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(123)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 9, 240, 123], OperandSize::Qword)
}

#[test]
fn vroundpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPD, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(89)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 9, 16, 89], OperandSize::Qword)
}

#[test]
fn vroundpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(Literal8(92)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 9, 208, 92], OperandSize::Dword)
}

#[test]
fn vroundpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPD, operand1: Some(Direct(YMM2)), operand2: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(99)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 9, 23, 99], OperandSize::Dword)
}

#[test]
fn vroundpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(Literal8(94)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 9, 202, 94], OperandSize::Qword)
}

#[test]
fn vroundpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPD, operand1: Some(Direct(YMM6)), operand2: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(92)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 9, 51, 92], OperandSize::Qword)
}

