use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vaddpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 88, 251], OperandSize::Dword)
}

#[test]
fn vaddpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Two, 1587493453, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 88, 172, 83, 77, 58, 159, 94], OperandSize::Dword)
}

#[test]
fn vaddpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 88, 232], OperandSize::Qword)
}

#[test]
fn vaddpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(RCX, 2002132945, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 88, 137, 209, 31, 86, 119], OperandSize::Qword)
}

#[test]
fn vaddpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 88, 213], OperandSize::Dword)
}

#[test]
fn vaddpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 88, 56], OperandSize::Dword)
}

#[test]
fn vaddpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 88, 254], OperandSize::Qword)
}

#[test]
fn vaddpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(RCX, 360798031, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 88, 129, 79, 87, 129, 21], OperandSize::Qword)
}

#[test]
fn vaddpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 237, 141, 88, 234], OperandSize::Dword)
}

#[test]
fn vaddpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 197, 138, 88, 0], OperandSize::Dword)
}

#[test]
fn vaddpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 1779384287, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 197, 155, 88, 28, 189, 223, 63, 15, 106], OperandSize::Dword)
}

#[test]
fn vaddpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM10)), operand3: Some(Direct(XMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 129, 173, 139, 88, 216], OperandSize::Qword)
}

#[test]
fn vaddpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 205, 140, 88, 50], OperandSize::Qword)
}

#[test]
fn vaddpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 1426913848, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 213, 156, 88, 52, 69, 56, 250, 12, 85], OperandSize::Qword)
}

#[test]
fn vaddpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 253, 173, 88, 235], OperandSize::Dword)
}

#[test]
fn vaddpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 197, 170, 88, 23], OperandSize::Dword)
}

#[test]
fn vaddpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 229, 187, 88, 25], OperandSize::Dword)
}

#[test]
fn vaddpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM19)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 229, 161, 88, 202], OperandSize::Qword)
}

#[test]
fn vaddpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM19)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 656770395, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 229, 163, 88, 52, 133, 91, 133, 37, 39], OperandSize::Qword)
}

#[test]
fn vaddpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM31)), operand3: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 97, 133, 180, 88, 17], OperandSize::Qword)
}

#[test]
fn vaddpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 229, 250, 88, 223], OperandSize::Dword)
}

#[test]
fn vaddpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexed(EDI, EAX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 205, 203, 88, 36, 135], OperandSize::Dword)
}

#[test]
fn vaddpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectDisplaced(ECX, 802055242, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 197, 223, 88, 169, 74, 100, 206, 47], OperandSize::Dword)
}

#[test]
fn vaddpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM21)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 177, 253, 220, 88, 205], OperandSize::Qword)
}

#[test]
fn vaddpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM23)), operand3: Some(IndirectScaledIndexed(RDX, RBX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 197, 193, 88, 44, 218], OperandSize::Qword)
}

#[test]
fn vaddpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectDisplaced(RSI, 1322845769, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 237, 217, 88, 158, 73, 6, 217, 78], OperandSize::Qword)
}

