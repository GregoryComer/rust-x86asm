use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vroundpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(72)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 9, 202, 72], OperandSize::Dword)
}

#[test]
fn vroundpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(ESI, 945757862, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(9)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 9, 142, 166, 30, 95, 56, 9], OperandSize::Dword)
}

#[test]
fn vroundpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(90)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 9, 192, 90], OperandSize::Qword)
}

#[test]
fn vroundpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 1365622230, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(28)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 9, 44, 69, 214, 189, 101, 81, 28], OperandSize::Qword)
}

#[test]
fn vroundpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(Literal8(76)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 9, 202, 76], OperandSize::Dword)
}

#[test]
fn vroundpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPD, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 355554261, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(79)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 9, 20, 253, 213, 83, 49, 21, 79], OperandSize::Dword)
}

#[test]
fn vroundpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Literal8(61)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 9, 209, 61], OperandSize::Qword)
}

#[test]
fn vroundpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPD, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledIndexed(RCX, RSI, Two, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(55)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 9, 28, 113, 55], OperandSize::Qword)
}

