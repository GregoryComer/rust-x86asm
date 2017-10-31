use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpuq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM1)), operand4: Some(Literal8(18)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 197, 14, 30, 233, 18], OperandSize::Dword)
}

#[test]
fn vpcmpuq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Eight, 1051744025, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(119)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 221, 10, 30, 188, 208, 25, 87, 176, 62, 119], OperandSize::Dword)
}

#[test]
fn vpcmpuq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand4: Some(Literal8(95)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 229, 26, 30, 34, 95], OperandSize::Dword)
}

#[test]
fn vpcmpuq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM12)), operand3: Some(Direct(XMM0)), operand4: Some(Literal8(108)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 157, 9, 30, 232, 108], OperandSize::Qword)
}

#[test]
fn vpcmpuq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM16)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 50830259, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(124)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 253, 7, 30, 44, 197, 179, 155, 7, 3, 124], OperandSize::Qword)
}

#[test]
fn vpcmpuq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM16)), operand3: Some(IndirectScaledIndexed(RAX, RSI, Four, Some(OperandSize::Qword), None)), operand4: Some(Literal8(92)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 253, 21, 30, 36, 176, 92], OperandSize::Qword)
}

#[test]
fn vpcmpuq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM7)), operand4: Some(Literal8(86)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 245, 43, 30, 239, 86], OperandSize::Dword)
}

#[test]
fn vpcmpuq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(44)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 221, 47, 30, 11, 44], OperandSize::Dword)
}

#[test]
fn vpcmpuq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand4: Some(Literal8(46)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 197, 58, 30, 47, 46], OperandSize::Dword)
}

#[test]
fn vpcmpuq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM15)), operand4: Some(Literal8(67)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 211, 205, 44, 30, 223, 67], OperandSize::Qword)
}

#[test]
fn vpcmpuq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM20)), operand3: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(104)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 221, 34, 30, 42, 104], OperandSize::Qword)
}

#[test]
fn vpcmpuq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand4: Some(Literal8(117)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 237, 59, 30, 26, 117], OperandSize::Qword)
}

#[test]
fn vpcmpuq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM0)), operand4: Some(Literal8(37)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 253, 74, 30, 240, 37], OperandSize::Dword)
}

#[test]
fn vpcmpuq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Two, 375577829, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(108)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 245, 76, 30, 172, 89, 229, 220, 98, 22, 108], OperandSize::Dword)
}

#[test]
fn vpcmpuq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Two, 338773926, Some(OperandSize::Qword), None)), operand4: Some(Literal8(14)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 205, 91, 30, 172, 80, 166, 71, 49, 20, 14], OperandSize::Dword)
}

#[test]
fn vpcmpuq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM11)), operand3: Some(Direct(ZMM31)), operand4: Some(Literal8(8)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 147, 165, 75, 30, 231, 8], OperandSize::Qword)
}

#[test]
fn vpcmpuq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM18)), operand3: Some(IndirectDisplaced(RCX, 1501914934, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(70)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 237, 68, 30, 177, 54, 103, 133, 89, 70], OperandSize::Qword)
}

#[test]
fn vpcmpuq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexed(RDI, RAX, Four, Some(OperandSize::Qword), None)), operand4: Some(Literal8(60)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 213, 89, 30, 60, 135, 60], OperandSize::Qword)
}

