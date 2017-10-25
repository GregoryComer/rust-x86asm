use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpuq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: Some(Literal8(85)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 237, 12, 30, 225, 85], OperandSize::Dword)
}

#[test]
fn vpcmpuq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(61)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 237, 10, 30, 46, 61], OperandSize::Dword)
}

#[test]
fn vpcmpuq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(EAX, 1467541434, Some(OperandSize::Qword), None)), operand4: Some(Literal8(39)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 221, 30, 30, 160, 186, 231, 120, 87, 39], OperandSize::Dword)
}

#[test]
fn vpcmpuq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM25)), operand3: Some(Direct(XMM5)), operand4: Some(Literal8(73)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 181, 3, 30, 221, 73], OperandSize::Qword)
}

#[test]
fn vpcmpuq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM28)), operand3: Some(IndirectDisplaced(RDX, 927305415, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(114)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 157, 6, 30, 162, 199, 142, 69, 55, 114], OperandSize::Qword)
}

#[test]
fn vpcmpuq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM16)), operand3: Some(IndirectDisplaced(RAX, 735553966, Some(OperandSize::Qword), None)), operand4: Some(Literal8(103)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 253, 21, 30, 184, 174, 169, 215, 43, 103], OperandSize::Qword)
}

#[test]
fn vpcmpuq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM4)), operand4: Some(Literal8(51)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 197, 43, 30, 220, 51], OperandSize::Dword)
}

#[test]
fn vpcmpuq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 11945730, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(6)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 245, 44, 30, 44, 245, 2, 71, 182, 0, 6], OperandSize::Dword)
}

#[test]
fn vpcmpuq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(EDI, 70596843, Some(OperandSize::Qword), None)), operand4: Some(Literal8(90)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 237, 60, 30, 175, 235, 56, 53, 4, 90], OperandSize::Dword)
}

#[test]
fn vpcmpuq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM9)), operand3: Some(Direct(YMM25)), operand4: Some(Literal8(12)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 147, 181, 41, 30, 249, 12], OperandSize::Qword)
}

#[test]
fn vpcmpuq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM26)), operand3: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(72)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 173, 38, 30, 23, 72], OperandSize::Qword)
}

#[test]
fn vpcmpuq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM25)), operand3: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand4: Some(Literal8(7)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 181, 49, 30, 24, 7], OperandSize::Qword)
}

#[test]
fn vpcmpuq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM0)), operand4: Some(Literal8(117)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 221, 76, 30, 224, 117], OperandSize::Dword)
}

#[test]
fn vpcmpuq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 350463209, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(39)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 245, 77, 30, 44, 189, 233, 164, 227, 20, 39], OperandSize::Dword)
}

#[test]
fn vpcmpuq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Two, 991217843, Some(OperandSize::Qword), None)), operand4: Some(Literal8(37)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 197, 95, 30, 180, 91, 179, 200, 20, 59, 37], OperandSize::Dword)
}

#[test]
fn vpcmpuq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM24)), operand3: Some(Direct(ZMM11)), operand4: Some(Literal8(19)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 211, 189, 67, 30, 251, 19], OperandSize::Qword)
}

#[test]
fn vpcmpuq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM9)), operand3: Some(IndirectDisplaced(RAX, 1817013785, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(106)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 181, 75, 30, 184, 25, 110, 77, 108, 106], OperandSize::Qword)
}

#[test]
fn vpcmpuq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM23)), operand3: Some(IndirectDisplaced(RDI, 692355903, Some(OperandSize::Qword), None)), operand4: Some(Literal8(1)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 197, 81, 30, 191, 63, 131, 68, 41, 1], OperandSize::Qword)
}

