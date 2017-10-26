use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpternlogd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM1)), operand4: Some(Literal8(99)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 93, 138, 37, 241, 99], OperandSize::Dword)
}

#[test]
fn vpternlogd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(45)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 109, 138, 37, 33, 45], OperandSize::Dword)
}

#[test]
fn vpternlogd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 374378646, Some(OperandSize::Dword), None)), operand4: Some(Literal8(109)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 93, 156, 37, 44, 93, 150, 144, 80, 22, 109], OperandSize::Dword)
}

#[test]
fn vpternlogd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM19)), operand3: Some(Direct(XMM5)), operand4: Some(Literal8(84)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 101, 132, 37, 221, 84], OperandSize::Qword)
}

#[test]
fn vpternlogd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Four, 344569350, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(26)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 109, 140, 37, 140, 154, 6, 182, 137, 20, 26], OperandSize::Qword)
}

#[test]
fn vpternlogd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM25)), operand3: Some(IndirectDisplaced(RSI, 515624071, Some(OperandSize::Dword), None)), operand4: Some(Literal8(22)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 115, 53, 150, 37, 134, 135, 204, 187, 30, 22], OperandSize::Qword)
}

#[test]
fn vpternlogd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM5)), operand4: Some(Literal8(104)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 109, 173, 37, 221, 104], OperandSize::Dword)
}

#[test]
fn vpternlogd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Eight, 2037351796, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(53)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 125, 175, 37, 180, 208, 116, 133, 111, 121, 53], OperandSize::Dword)
}

#[test]
fn vpternlogd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(EBX, 537458441, Some(OperandSize::Dword), None)), operand4: Some(Literal8(10)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 69, 185, 37, 171, 9, 247, 8, 32, 10], OperandSize::Dword)
}

#[test]
fn vpternlogd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM15)), operand3: Some(Direct(YMM3)), operand4: Some(Literal8(40)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 99, 5, 173, 37, 235, 40], OperandSize::Qword)
}

#[test]
fn vpternlogd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 1179917032, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(118)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 125, 175, 37, 44, 85, 232, 26, 84, 70, 118], OperandSize::Qword)
}

#[test]
fn vpternlogd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(RAX, RBX, Four, Some(OperandSize::Dword), None)), operand4: Some(Literal8(101)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 227, 85, 185, 37, 36, 152, 101], OperandSize::Qword)
}

#[test]
fn vpternlogd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM5)), operand4: Some(Literal8(23)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 117, 202, 37, 221, 23], OperandSize::Dword)
}

#[test]
fn vpternlogd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 1983006210, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(116)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 85, 204, 37, 52, 197, 2, 70, 50, 118, 116], OperandSize::Dword)
}

#[test]
fn vpternlogd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand4: Some(Literal8(108)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 69, 218, 37, 24, 108], OperandSize::Dword)
}

#[test]
fn vpternlogd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM24)), operand3: Some(Direct(ZMM26)), operand4: Some(Literal8(127)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 19, 61, 193, 37, 210, 127], OperandSize::Qword)
}

#[test]
fn vpternlogd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM19)), operand3: Some(IndirectDisplaced(RCX, 1240480536, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(83)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 101, 194, 37, 169, 24, 59, 240, 73, 83], OperandSize::Qword)
}

#[test]
fn vpternlogd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM15)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Four, 1520498561, Some(OperandSize::Dword), None)), operand4: Some(Literal8(112)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 227, 5, 217, 37, 180, 136, 129, 247, 160, 90, 112], OperandSize::Qword)
}

