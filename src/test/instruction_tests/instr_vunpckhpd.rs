use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vunpckhpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 21, 241], OperandSize::Dword)
}

#[test]
fn vunpckhpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(ECX, EDI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 21, 20, 121], OperandSize::Dword)
}

#[test]
fn vunpckhpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 21, 196], OperandSize::Qword)
}

#[test]
fn vunpckhpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(RDX, RSI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 21, 36, 114], OperandSize::Qword)
}

#[test]
fn vunpckhpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 21, 254], OperandSize::Dword)
}

#[test]
fn vunpckhpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(ESI, EDX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 21, 12, 150], OperandSize::Dword)
}

#[test]
fn vunpckhpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 21, 216], OperandSize::Qword)
}

#[test]
fn vunpckhpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(RSI, RDX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 21, 20, 86], OperandSize::Qword)
}

#[test]
fn vunpckhpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 205, 140, 21, 252], OperandSize::Dword)
}

#[test]
fn vunpckhpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(ESI, ESI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 237, 137, 21, 20, 246], OperandSize::Dword)
}

#[test]
fn vunpckhpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Two, 109061417, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 221, 156, 21, 188, 71, 41, 37, 128, 6], OperandSize::Dword)
}

#[test]
fn vunpckhpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM14)), operand3: Some(Direct(XMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 193, 141, 139, 21, 227], OperandSize::Qword)
}

#[test]
fn vunpckhpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM22)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 205, 130, 21, 22], OperandSize::Qword)
}

#[test]
fn vunpckhpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM27)), operand3: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 165, 148, 21, 58], OperandSize::Qword)
}

#[test]
fn vunpckhpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 197, 173, 21, 210], OperandSize::Dword)
}

#[test]
fn vunpckhpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 253, 173, 21, 7], OperandSize::Dword)
}

#[test]
fn vunpckhpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 1371742129, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 221, 190, 21, 12, 149, 177, 31, 195, 81], OperandSize::Dword)
}

#[test]
fn vunpckhpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM24)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 189, 162, 21, 213], OperandSize::Qword)
}

#[test]
fn vunpckhpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 1731775454, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 213, 173, 21, 44, 157, 222, 203, 56, 103], OperandSize::Qword)
}

#[test]
fn vunpckhpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 1317685821, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 97, 229, 188, 21, 12, 85, 61, 74, 138, 78], OperandSize::Qword)
}

#[test]
fn vunpckhpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 204, 21, 207], OperandSize::Dword)
}

#[test]
fn vunpckhpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Two, 170189621, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 207, 21, 132, 72, 53, 227, 36, 10], OperandSize::Dword)
}

#[test]
fn vunpckhpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 1522237591, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 229, 220, 21, 60, 141, 151, 128, 187, 90], OperandSize::Dword)
}

#[test]
fn vunpckhpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM10)), operand3: Some(Direct(ZMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 161, 173, 207, 21, 252], OperandSize::Qword)
}

#[test]
fn vunpckhpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectDisplaced(RDX, 1760699040, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 197, 201, 21, 170, 160, 34, 242, 104], OperandSize::Qword)
}

#[test]
fn vunpckhpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM28)), operand3: Some(IndirectDisplaced(RAX, 1458918767, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 113, 157, 211, 21, 176, 111, 85, 245, 86], OperandSize::Qword)
}

