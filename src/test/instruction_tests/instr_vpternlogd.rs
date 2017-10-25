use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpternlogd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: Some(Literal8(13)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 117, 142, 37, 219, 13], OperandSize::Dword)
}

#[test]
fn vpternlogd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(ESI, 2081206135, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(26)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 101, 139, 37, 150, 119, 175, 12, 124, 26], OperandSize::Dword)
}

#[test]
fn vpternlogd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(EBX, 375720857, Some(OperandSize::Dword), None)), operand4: Some(Literal8(52)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 101, 153, 37, 147, 153, 11, 101, 22, 52], OperandSize::Dword)
}

#[test]
fn vpternlogd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM25)), operand4: Some(Literal8(66)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 131, 85, 139, 37, 233, 66], OperandSize::Qword)
}

#[test]
fn vpternlogd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM13)), operand3: Some(IndirectDisplaced(RSI, 1512172990, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(56)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 99, 21, 143, 37, 142, 190, 237, 33, 90, 56], OperandSize::Qword)
}

#[test]
fn vpternlogd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM14)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Eight, 891852319, Some(OperandSize::Dword), None)), operand4: Some(Literal8(69)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 227, 13, 156, 37, 132, 202, 31, 150, 40, 53, 69], OperandSize::Qword)
}

#[test]
fn vpternlogd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM3)), operand4: Some(Literal8(96)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 85, 171, 37, 227, 96], OperandSize::Dword)
}

#[test]
fn vpternlogd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(EDX, 436944371, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(0)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 101, 171, 37, 146, 243, 61, 11, 26, 0], OperandSize::Dword)
}

#[test]
fn vpternlogd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(EAX, ECX, Eight, Some(OperandSize::Dword), None)), operand4: Some(Literal8(52)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 69, 190, 37, 20, 200, 52], OperandSize::Dword)
}

#[test]
fn vpternlogd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM26)), operand3: Some(Direct(YMM10)), operand4: Some(Literal8(47)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 211, 45, 165, 37, 242, 47], OperandSize::Qword)
}

#[test]
fn vpternlogd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM21)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Eight, 2010086403, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(46)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 85, 165, 37, 140, 246, 3, 124, 207, 119, 46], OperandSize::Qword)
}

#[test]
fn vpternlogd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM18)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Eight, 1070264129, Some(OperandSize::Dword), None)), operand4: Some(Literal8(95)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 109, 180, 37, 164, 216, 65, 239, 202, 63, 95], OperandSize::Qword)
}

#[test]
fn vpternlogd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM1)), operand4: Some(Literal8(36)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 125, 201, 37, 241, 36], OperandSize::Dword)
}

#[test]
fn vpternlogd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM6)), operand3: Some(Indirect(EBX, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(94)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 77, 207, 37, 19, 94], OperandSize::Dword)
}

#[test]
fn vpternlogd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 367219783, Some(OperandSize::Dword), None)), operand4: Some(Literal8(44)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 85, 217, 37, 12, 133, 71, 84, 227, 21, 44], OperandSize::Dword)
}

#[test]
fn vpternlogd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM17)), operand3: Some(Direct(ZMM25)), operand4: Some(Literal8(32)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 131, 117, 193, 37, 233, 32], OperandSize::Qword)
}

#[test]
fn vpternlogd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM9)), operand3: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(19)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 99, 53, 203, 37, 42, 19], OperandSize::Qword)
}

#[test]
fn vpternlogd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM15)), operand3: Some(IndirectScaledIndexed(RDI, RBX, Four, Some(OperandSize::Dword), None)), operand4: Some(Literal8(21)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 115, 5, 219, 37, 28, 159, 21], OperandSize::Qword)
}

