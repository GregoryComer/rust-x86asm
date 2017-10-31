use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpblendd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM3)), operand4: Some(Literal8(1)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 81, 2, 219, 1], OperandSize::Dword)
}

#[test]
fn vpblendd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Four, 995628453, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(72)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 89, 2, 148, 184, 165, 21, 88, 59, 72], OperandSize::Dword)
}

#[test]
fn vpblendd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM5)), operand4: Some(Literal8(48)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 2, 213, 48], OperandSize::Qword)
}

#[test]
fn vpblendd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(85)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 2, 25, 85], OperandSize::Qword)
}

#[test]
fn vpblendd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM6)), operand4: Some(Literal8(70)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 109, 2, 246, 70], OperandSize::Dword)
}

#[test]
fn vpblendd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Two, 1496355775, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(57)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 117, 2, 148, 79, 191, 147, 48, 89, 57], OperandSize::Dword)
}

#[test]
fn vpblendd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM7)), operand4: Some(Literal8(9)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 109, 2, 199, 9], OperandSize::Qword)
}

#[test]
fn vpblendd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Four, 1707076465, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(22)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 109, 2, 148, 190, 113, 235, 191, 101, 22], OperandSize::Qword)
}

