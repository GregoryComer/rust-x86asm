use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: Some(Literal8(54)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 69, 9, 31, 224, 54], OperandSize::Dword)
}

#[test]
fn vpcmpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(ESI, ECX, Four, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(103)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 101, 12, 31, 60, 142, 103], OperandSize::Dword)
}

#[test]
fn vpcmpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(EDI, 1481728402, Some(OperandSize::Dword), None)), operand4: Some(Literal8(116)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 109, 28, 31, 159, 146, 97, 81, 88, 116], OperandSize::Dword)
}

#[test]
fn vpcmpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM11)), operand4: Some(Literal8(83)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 211, 117, 14, 31, 219, 83], OperandSize::Qword)
}

#[test]
fn vpcmpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM26)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(100)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 45, 6, 31, 54, 100], OperandSize::Qword)
}

#[test]
fn vpcmpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand4: Some(Literal8(116)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 69, 31, 31, 9, 116], OperandSize::Qword)
}

#[test]
fn vpcmpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM6)), operand4: Some(Literal8(100)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 69, 47, 31, 254, 100], OperandSize::Dword)
}

#[test]
fn vpcmpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(EDX, 1133858001, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(39)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 101, 47, 31, 178, 209, 76, 149, 67, 39], OperandSize::Dword)
}

#[test]
fn vpcmpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand4: Some(Literal8(123)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 125, 63, 31, 51, 123], OperandSize::Dword)
}

#[test]
fn vpcmpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM14)), operand3: Some(Direct(YMM7)), operand4: Some(Literal8(118)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 13, 43, 31, 215, 118], OperandSize::Qword)
}

#[test]
fn vpcmpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM10)), operand3: Some(IndirectDisplaced(RAX, 574761012, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(100)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 45, 47, 31, 144, 52, 40, 66, 34, 100], OperandSize::Qword)
}

#[test]
fn vpcmpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM9)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 1853734496, Some(OperandSize::Dword), None)), operand4: Some(Literal8(100)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 53, 60, 31, 12, 93, 96, 190, 125, 110, 100], OperandSize::Qword)
}

#[test]
fn vpcmpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM1)), operand4: Some(Literal8(23)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 69, 75, 31, 241, 23], OperandSize::Dword)
}

#[test]
fn vpcmpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexed(ESI, ECX, Two, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(92)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 101, 74, 31, 60, 78, 92], OperandSize::Dword)
}

#[test]
fn vpcmpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 1640392784, Some(OperandSize::Dword), None)), operand4: Some(Literal8(96)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 93, 92, 31, 28, 205, 80, 104, 198, 97, 96], OperandSize::Dword)
}

#[test]
fn vpcmpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM20)), operand4: Some(Literal8(52)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 179, 101, 73, 31, 220, 52], OperandSize::Qword)
}

#[test]
fn vpcmpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM25)), operand3: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(73)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 53, 70, 31, 18, 73], OperandSize::Qword)
}

#[test]
fn vpcmpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM31)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 1932621267, Some(OperandSize::Dword), None)), operand4: Some(Literal8(10)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 5, 86, 31, 20, 213, 211, 117, 49, 115, 10], OperandSize::Qword)
}

