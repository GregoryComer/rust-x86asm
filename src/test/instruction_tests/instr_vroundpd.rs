use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vroundpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(92)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 9, 241, 92], OperandSize::Dword)
}

#[test]
fn vroundpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Four, 1462866570, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(114)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 9, 148, 152, 138, 146, 49, 87, 114], OperandSize::Dword)
}

#[test]
fn vroundpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(112)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 9, 205, 112], OperandSize::Qword)
}

#[test]
fn vroundpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(RDX, 313822761, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(95)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 9, 130, 41, 142, 180, 18, 95], OperandSize::Qword)
}

#[test]
fn vroundpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(Literal8(88)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 9, 203, 88], OperandSize::Dword)
}

#[test]
fn vroundpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPD, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledIndexed(EDX, ESI, Eight, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(51)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 9, 60, 242, 51], OperandSize::Dword)
}

#[test]
fn vroundpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(97)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 9, 237, 97], OperandSize::Qword)
}

#[test]
fn vroundpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPD, operand1: Some(Direct(YMM5)), operand2: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(119)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 9, 47, 119], OperandSize::Qword)
}

