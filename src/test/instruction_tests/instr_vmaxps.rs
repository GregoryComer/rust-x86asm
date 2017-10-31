use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmaxps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 224, 95, 193], OperandSize::Dword)
}

#[test]
fn vmaxps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 95, 55], OperandSize::Dword)
}

#[test]
fn vmaxps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 216, 95, 206], OperandSize::Qword)
}

#[test]
fn vmaxps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(RSI, RDI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 192, 95, 20, 190], OperandSize::Qword)
}

#[test]
fn vmaxps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 244, 95, 215], OperandSize::Dword)
}

#[test]
fn vmaxps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 228, 95, 24], OperandSize::Dword)
}

#[test]
fn vmaxps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 196, 95, 244], OperandSize::Qword)
}

#[test]
fn vmaxps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(RBX, 185359195, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 196, 95, 179, 91, 91, 12, 11], OperandSize::Qword)
}

#[test]
fn vmaxps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 92, 138, 95, 208], OperandSize::Dword)
}

#[test]
fn vmaxps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(ESI, 1089217348, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 108, 140, 95, 142, 68, 35, 236, 64], OperandSize::Dword)
}

#[test]
fn vmaxps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 1433624577, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 100, 158, 95, 36, 189, 1, 96, 115, 85], OperandSize::Dword)
}

#[test]
fn vmaxps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM29)), operand3: Some(Direct(XMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 209, 20, 134, 95, 255], OperandSize::Qword)
}

#[test]
fn vmaxps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM24)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Two, 215237879, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 60, 130, 95, 140, 66, 247, 68, 212, 12], OperandSize::Qword)
}

#[test]
fn vmaxps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM9)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Eight, 505558736, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 52, 155, 95, 156, 223, 208, 54, 34, 30], OperandSize::Qword)
}

#[test]
fn vmaxps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 124, 172, 95, 207], OperandSize::Dword)
}

#[test]
fn vmaxps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Two, 1277850490, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 68, 173, 95, 180, 113, 122, 115, 42, 76], OperandSize::Dword)
}

#[test]
fn vmaxps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Two, 24276579, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 100, 185, 95, 172, 126, 99, 110, 114, 1], OperandSize::Dword)
}

#[test]
fn vmaxps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 1, 92, 173, 95, 195], OperandSize::Qword)
}

#[test]
fn vmaxps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM12)), operand3: Some(IndirectScaledIndexed(RBX, RSI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 28, 174, 95, 4, 179], OperandSize::Qword)
}

#[test]
fn vmaxps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM24)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Two, 954396686, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 60, 181, 95, 132, 83, 14, 240, 226, 56], OperandSize::Qword)
}

#[test]
fn vmaxps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 100, 157, 95, 212], OperandSize::Dword)
}

#[test]
fn vmaxps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM5)), operand3: Some(Indirect(EDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 84, 207, 95, 10], OperandSize::Dword)
}

#[test]
fn vmaxps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 1693739019, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 116, 222, 95, 28, 125, 11, 104, 244, 100], OperandSize::Dword)
}

#[test]
fn vmaxps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM21)), operand3: Some(Direct(ZMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 129, 84, 146, 95, 248], OperandSize::Qword)
}

#[test]
fn vmaxps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM13)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 1776064181, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 20, 207, 95, 28, 141, 181, 150, 220, 105], OperandSize::Qword)
}

#[test]
fn vmaxps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM16)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 1510477081, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 124, 213, 95, 4, 197, 25, 13, 8, 90], OperandSize::Qword)
}

