use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmaxpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 95, 204], OperandSize::Dword)
}

#[test]
fn vmaxpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(EBX, EAX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 95, 4, 195], OperandSize::Dword)
}

#[test]
fn vmaxpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 95, 222], OperandSize::Qword)
}

#[test]
fn vmaxpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 1654738657, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 95, 44, 85, 225, 78, 161, 98], OperandSize::Qword)
}

#[test]
fn vmaxpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 95, 225], OperandSize::Dword)
}

#[test]
fn vmaxpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Eight, 1384045656, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 95, 188, 251, 88, 220, 126, 82], OperandSize::Dword)
}

#[test]
fn vmaxpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 95, 227], OperandSize::Qword)
}

#[test]
fn vmaxpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 95, 19], OperandSize::Qword)
}

#[test]
fn vmaxpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 213, 141, 95, 218], OperandSize::Dword)
}

#[test]
fn vmaxpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 400450353, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 197, 140, 95, 44, 149, 49, 99, 222, 23], OperandSize::Dword)
}

#[test]
fn vmaxpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 1667165203, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 229, 157, 95, 52, 141, 19, 236, 94, 99], OperandSize::Dword)
}

#[test]
fn vmaxpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM27)), operand3: Some(Direct(XMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 177, 165, 134, 95, 236], OperandSize::Qword)
}

#[test]
fn vmaxpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM25)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 181, 133, 95, 51], OperandSize::Qword)
}

#[test]
fn vmaxpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM24)), operand3: Some(IndirectScaledIndexed(RDX, RDI, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 189, 145, 95, 60, 250], OperandSize::Qword)
}

#[test]
fn vmaxpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 221, 171, 95, 207], OperandSize::Dword)
}

#[test]
fn vmaxpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(EBX, EBX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 205, 174, 95, 4, 219], OperandSize::Dword)
}

#[test]
fn vmaxpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(EDX, EAX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 229, 187, 95, 28, 130], OperandSize::Dword)
}

#[test]
fn vmaxpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 33, 213, 173, 95, 224], OperandSize::Qword)
}

#[test]
fn vmaxpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM16)), operand3: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 253, 167, 95, 56], OperandSize::Qword)
}

#[test]
fn vmaxpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 1621744281, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 97, 229, 190, 95, 36, 189, 153, 218, 169, 96], OperandSize::Qword)
}

#[test]
fn vmaxpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 213, 156, 95, 206], OperandSize::Dword)
}

#[test]
fn vmaxpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM7)), operand3: Some(Indirect(EDI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 197, 207, 95, 47], OperandSize::Dword)
}

#[test]
fn vmaxpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM5)), operand3: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 213, 219, 95, 57], OperandSize::Dword)
}

#[test]
fn vmaxpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 161, 197, 155, 95, 212], OperandSize::Qword)
}

#[test]
fn vmaxpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM24)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 1242231017, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 189, 195, 95, 44, 245, 233, 240, 10, 74], OperandSize::Qword)
}

#[test]
fn vmaxpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM15)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Eight, 1198581773, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 113, 133, 221, 95, 148, 199, 13, 232, 112, 71], OperandSize::Qword)
}

