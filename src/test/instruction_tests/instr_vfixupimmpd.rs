use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfixupimmpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(51)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 213, 138, 84, 204, 51], OperandSize::Dword)
}

#[test]
fn vfixupimmpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(122)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 213, 138, 84, 6, 122], OperandSize::Dword)
}

#[test]
fn vfixupimmpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand4: Some(Literal8(51)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 237, 154, 84, 27, 51], OperandSize::Dword)
}

#[test]
fn vfixupimmpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM12)), operand3: Some(Direct(XMM22)), operand4: Some(Literal8(36)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 179, 157, 138, 84, 206, 36], OperandSize::Qword)
}

#[test]
fn vfixupimmpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM14)), operand3: Some(IndirectDisplaced(RCX, 1434290728, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(93)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 115, 141, 140, 84, 137, 40, 138, 125, 85, 93], OperandSize::Qword)
}

#[test]
fn vfixupimmpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM9)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Two, 203166326, Some(OperandSize::Qword), None)), operand4: Some(Literal8(99)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 99, 181, 154, 84, 188, 86, 118, 18, 28, 12, 99], OperandSize::Qword)
}

#[test]
fn vfixupimmpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM1)), operand4: Some(Literal8(113)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 205, 169, 84, 249, 113], OperandSize::Dword)
}

#[test]
fn vfixupimmpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(EDI, 805936907, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(125)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 205, 173, 84, 191, 11, 159, 9, 48, 125], OperandSize::Dword)
}

#[test]
fn vfixupimmpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Four, 1402629826, Some(OperandSize::Qword), None)), operand4: Some(Literal8(64)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 221, 189, 84, 140, 178, 194, 110, 154, 83, 64], OperandSize::Dword)
}

#[test]
fn vfixupimmpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM8)), operand3: Some(Direct(YMM9)), operand4: Some(Literal8(8)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 67, 189, 171, 84, 233, 8], OperandSize::Qword)
}

#[test]
fn vfixupimmpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM9)), operand3: Some(IndirectDisplaced(RDI, 1810758570, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(108)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 181, 175, 84, 191, 170, 251, 237, 107, 108], OperandSize::Qword)
}

#[test]
fn vfixupimmpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 642938398, Some(OperandSize::Qword), None)), operand4: Some(Literal8(44)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 115, 229, 185, 84, 12, 221, 30, 118, 82, 38, 44], OperandSize::Qword)
}

#[test]
fn vfixupimmpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM5)), operand4: Some(Literal8(74)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 253, 157, 84, 245, 74], OperandSize::Dword)
}

#[test]
fn vfixupimmpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectDisplaced(EDI, 792494438, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(32)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 213, 204, 84, 151, 102, 129, 60, 47, 32], OperandSize::Dword)
}

#[test]
fn vfixupimmpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Eight, 1902387379, Some(OperandSize::Qword), None)), operand4: Some(Literal8(1)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 205, 219, 84, 172, 203, 179, 32, 100, 113, 1], OperandSize::Dword)
}

#[test]
fn vfixupimmpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM12)), operand3: Some(Direct(ZMM1)), operand4: Some(Literal8(2)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 115, 157, 156, 84, 233, 2], OperandSize::Qword)
}

#[test]
fn vfixupimmpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM31)), operand3: Some(IndirectDisplaced(RDI, 194248362, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(96)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 133, 198, 84, 183, 170, 254, 147, 11, 96], OperandSize::Qword)
}

#[test]
fn vfixupimmpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Two, 1196446697, Some(OperandSize::Qword), None)), operand4: Some(Literal8(108)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 99, 213, 219, 84, 156, 86, 233, 83, 80, 71, 108], OperandSize::Qword)
}

