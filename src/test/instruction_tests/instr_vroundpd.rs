use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vroundpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(116)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 9, 248, 116], OperandSize::Dword)
}

#[test]
fn vroundpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Four, 1596553812, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(64)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 9, 172, 184, 84, 122, 41, 95, 64], OperandSize::Dword)
}

#[test]
fn vroundpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(80)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 9, 216, 80], OperandSize::Qword)
}

#[test]
fn vroundpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Four, 272314045, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(60)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 9, 188, 183, 189, 46, 59, 16, 60], OperandSize::Qword)
}

#[test]
fn vroundpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(Literal8(95)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 9, 251, 95], OperandSize::Dword)
}

#[test]
fn vroundpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPD, operand1: Some(Direct(YMM0)), operand2: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(60)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 9, 7, 60], OperandSize::Dword)
}

#[test]
fn vroundpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Literal8(68)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 9, 233, 68], OperandSize::Qword)
}

#[test]
fn vroundpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPD, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 1079558185, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(10)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 9, 28, 205, 41, 192, 88, 64, 10], OperandSize::Qword)
}

