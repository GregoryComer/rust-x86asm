use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vgetmantps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(123)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 125, 142, 38, 200, 123], OperandSize::Dword)
}

#[test]
fn vgetmantps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(ESI, Eight, 1979926932, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(86)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 125, 139, 38, 44, 245, 148, 73, 3, 118, 86], OperandSize::Dword)
}

#[test]
fn vgetmantps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Four, 740376281, Some(OperandSize::Dword), None)), operand3: Some(Literal8(20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 125, 159, 38, 156, 179, 217, 62, 33, 44, 20], OperandSize::Dword)
}

#[test]
fn vgetmantps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(39)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 115, 125, 140, 38, 194, 39], OperandSize::Qword)
}

#[test]
fn vgetmantps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Eight, 1436109413, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(101)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 125, 139, 38, 148, 215, 101, 74, 153, 85, 101], OperandSize::Qword)
}

#[test]
fn vgetmantps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(XMM17)), operand2: Some(IndirectScaledIndexed(RCX, RCX, Four, Some(OperandSize::Dword), None)), operand3: Some(Literal8(64)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 227, 125, 158, 38, 12, 137, 64], OperandSize::Qword)
}

#[test]
fn vgetmantps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Literal8(123)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 125, 175, 38, 209, 123], OperandSize::Dword)
}

#[test]
fn vgetmantps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(YMM6)), operand2: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(61)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 125, 173, 38, 55, 61], OperandSize::Dword)
}

#[test]
fn vgetmantps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(YMM4)), operand2: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand3: Some(Literal8(103)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 125, 186, 38, 38, 103], OperandSize::Dword)
}

#[test]
fn vgetmantps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM9)), operand3: Some(Literal8(100)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 67, 125, 171, 38, 209, 100], OperandSize::Qword)
}

#[test]
fn vgetmantps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(YMM9)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Eight, 1211431759, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(106)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 115, 125, 172, 38, 140, 219, 79, 251, 52, 72, 106], OperandSize::Qword)
}

#[test]
fn vgetmantps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(YMM27)), operand2: Some(IndirectDisplaced(RDI, 1777836394, Some(OperandSize::Dword), None)), operand3: Some(Literal8(85)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 99, 125, 189, 38, 159, 106, 161, 247, 105, 85], OperandSize::Qword)
}

#[test]
fn vgetmantps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM0)), operand3: Some(Literal8(106)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 125, 153, 38, 240, 106], OperandSize::Dword)
}

#[test]
fn vgetmantps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(ZMM5)), operand2: Some(Indirect(EDI, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 125, 201, 38, 47, 11], OperandSize::Dword)
}

#[test]
fn vgetmantps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledIndexed(EDX, ESI, Eight, Some(OperandSize::Dword), None)), operand3: Some(Literal8(55)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 125, 219, 38, 60, 242, 55], OperandSize::Dword)
}

#[test]
fn vgetmantps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM2)), operand3: Some(Literal8(91)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 99, 125, 153, 38, 226, 91], OperandSize::Qword)
}

#[test]
fn vgetmantps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(ZMM27)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Eight, 970654224, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(51)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 99, 125, 206, 38, 156, 193, 16, 2, 219, 57, 51], OperandSize::Qword)
}

#[test]
fn vgetmantps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(ZMM13)), operand2: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 115, 125, 219, 38, 43, 9], OperandSize::Qword)
}

