use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPB, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(101)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 77, 15, 63, 234, 101], OperandSize::Dword)
}

#[test]
fn vpcmpb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPB, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Two, 381228499, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(57)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 117, 10, 63, 172, 66, 211, 21, 185, 22, 57], OperandSize::Dword)
}

#[test]
fn vpcmpb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPB, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM28)), operand3: Some(Direct(XMM1)), operand4: Some(Literal8(19)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 29, 4, 63, 225, 19], OperandSize::Qword)
}

#[test]
fn vpcmpb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPB, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Four, 356192790, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(102)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 77, 9, 63, 172, 138, 22, 18, 59, 21, 102], OperandSize::Qword)
}

#[test]
fn vpcmpb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPB, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM6)), operand4: Some(Literal8(57)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 85, 43, 63, 206, 57], OperandSize::Dword)
}

#[test]
fn vpcmpb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPB, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(ECX, EDX, Two, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(123)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 69, 43, 63, 28, 81, 123], OperandSize::Dword)
}

#[test]
fn vpcmpb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPB, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM25)), operand3: Some(Direct(YMM29)), operand4: Some(Literal8(75)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 147, 53, 39, 63, 205, 75], OperandSize::Qword)
}

#[test]
fn vpcmpb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPB, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM10)), operand3: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(63)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 45, 43, 63, 51, 63], OperandSize::Qword)
}

#[test]
fn vpcmpb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPB, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM6)), operand4: Some(Literal8(29)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 69, 73, 63, 214, 29], OperandSize::Dword)
}

#[test]
fn vpcmpb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPB, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexed(EDX, ECX, Eight, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(82)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 93, 78, 63, 28, 202, 82], OperandSize::Dword)
}

#[test]
fn vpcmpb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPB, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM24)), operand3: Some(Direct(ZMM18)), operand4: Some(Literal8(117)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 179, 61, 71, 63, 226, 117], OperandSize::Qword)
}

#[test]
fn vpcmpb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPB, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM22)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Eight, 399948383, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(95)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 77, 70, 63, 180, 206, 95, 186, 214, 23, 95], OperandSize::Qword)
}

