use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpminud_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 59, 237], OperandSize::Dword)
}

#[test]
fn vpminud_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(ECX, 858190606, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 59, 161, 14, 243, 38, 51], OperandSize::Dword)
}

#[test]
fn vpminud_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 59, 230], OperandSize::Qword)
}

#[test]
fn vpminud_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 1147538315, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 59, 52, 69, 139, 11, 102, 68], OperandSize::Qword)
}

#[test]
fn vpminud_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 59, 197], OperandSize::Dword)
}

#[test]
fn vpminud_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 59, 2], OperandSize::Dword)
}

#[test]
fn vpminud_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 59, 235], OperandSize::Qword)
}

#[test]
fn vpminud_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(RSI, RDI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 59, 28, 126], OperandSize::Qword)
}

#[test]
fn vpminud_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 93, 143, 59, 212], OperandSize::Dword)
}

#[test]
fn vpminud_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(EAX, EDX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 69, 139, 59, 28, 144], OperandSize::Dword)
}

#[test]
fn vpminud_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(EBX, 968068333, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 69, 154, 59, 131, 237, 140, 179, 57], OperandSize::Dword)
}

#[test]
fn vpminud_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM29)), operand3: Some(Direct(XMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 146, 21, 135, 59, 200], OperandSize::Qword)
}

#[test]
fn vpminud_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM8)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 61, 137, 59, 47], OperandSize::Qword)
}

#[test]
fn vpminud_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(RDX, 1303181335, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 85, 159, 59, 138, 23, 248, 172, 77], OperandSize::Qword)
}

#[test]
fn vpminud_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 109, 170, 59, 203], OperandSize::Dword)
}

#[test]
fn vpminud_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(EAX, ESI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 93, 172, 59, 44, 176], OperandSize::Dword)
}

#[test]
fn vpminud_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(EBX, 88465654, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 109, 185, 59, 131, 246, 224, 69, 5], OperandSize::Dword)
}

#[test]
fn vpminud_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 66, 85, 174, 59, 199], OperandSize::Qword)
}

#[test]
fn vpminud_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM16)), operand3: Some(IndirectDisplaced(RSI, 430114561, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 125, 162, 59, 134, 1, 7, 163, 25], OperandSize::Qword)
}

#[test]
fn vpminud_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM27)), operand3: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 37, 180, 59, 31], OperandSize::Qword)
}

#[test]
fn vpminud_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 85, 202, 59, 249], OperandSize::Dword)
}

#[test]
fn vpminud_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectDisplaced(EAX, 1578009118, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 93, 207, 59, 152, 30, 130, 14, 94], OperandSize::Dword)
}

#[test]
fn vpminud_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 77315609, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 109, 218, 59, 60, 133, 25, 190, 155, 4], OperandSize::Dword)
}

#[test]
fn vpminud_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM16)), operand3: Some(Direct(ZMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 210, 125, 197, 59, 209], OperandSize::Qword)
}

#[test]
fn vpminud_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM29)), operand3: Some(Indirect(RAX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 21, 195, 59, 24], OperandSize::Qword)
}

#[test]
fn vpminud_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM15)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 934817676, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 5, 222, 59, 36, 77, 140, 47, 184, 55], OperandSize::Qword)
}

