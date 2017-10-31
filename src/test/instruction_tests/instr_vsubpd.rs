use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vsubpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 92, 238], OperandSize::Dword)
}

#[test]
fn vsubpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 92, 43], OperandSize::Dword)
}

#[test]
fn vsubpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 92, 195], OperandSize::Qword)
}

#[test]
fn vsubpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 69678614, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 92, 20, 157, 22, 54, 39, 4], OperandSize::Qword)
}

#[test]
fn vsubpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 92, 224], OperandSize::Dword)
}

#[test]
fn vsubpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 230220892, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 92, 12, 221, 92, 228, 184, 13], OperandSize::Dword)
}

#[test]
fn vsubpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 92, 224], OperandSize::Qword)
}

#[test]
fn vsubpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(RBX, RCX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 92, 20, 75], OperandSize::Qword)
}

#[test]
fn vsubpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 237, 140, 92, 241], OperandSize::Dword)
}

#[test]
fn vsubpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 1542299739, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 229, 137, 92, 44, 221, 91, 160, 237, 91], OperandSize::Dword)
}

#[test]
fn vsubpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 213, 156, 92, 26], OperandSize::Dword)
}

#[test]
fn vsubpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM28)), operand3: Some(Direct(XMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 49, 157, 134, 92, 192], OperandSize::Qword)
}

#[test]
fn vsubpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Four, 454691718, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 237, 138, 92, 140, 154, 134, 11, 26, 27], OperandSize::Qword)
}

#[test]
fn vsubpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM21)), operand3: Some(IndirectDisplaced(RSI, 719118342, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 225, 213, 149, 92, 134, 6, 224, 220, 42], OperandSize::Qword)
}

#[test]
fn vsubpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 205, 175, 92, 214], OperandSize::Dword)
}

#[test]
fn vsubpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Two, 999694156, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 205, 170, 92, 164, 65, 76, 31, 150, 59], OperandSize::Dword)
}

#[test]
fn vsubpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(EAX, EDI, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 221, 185, 92, 36, 120], OperandSize::Dword)
}

#[test]
fn vsubpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM17)), operand3: Some(Direct(YMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 1, 245, 163, 92, 202], OperandSize::Qword)
}

#[test]
fn vsubpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM15)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Four, 1391034687, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 133, 174, 92, 132, 176, 63, 129, 233, 82], OperandSize::Qword)
}

#[test]
fn vsubpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM14)), operand3: Some(IndirectDisplaced(RCX, 558779749, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 141, 189, 92, 129, 101, 77, 78, 33], OperandSize::Qword)
}

#[test]
fn vsubpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 245, 156, 92, 246], OperandSize::Dword)
}

#[test]
fn vsubpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectDisplaced(ESI, 322581603, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 221, 204, 92, 150, 99, 52, 58, 19], OperandSize::Dword)
}

#[test]
fn vsubpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 550314591, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 245, 221, 92, 36, 77, 95, 34, 205, 32], OperandSize::Dword)
}

#[test]
fn vsubpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM24)), operand3: Some(Direct(ZMM21)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 33, 189, 179, 92, 213], OperandSize::Qword)
}

#[test]
fn vsubpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM8)), operand3: Some(Indirect(RSI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 189, 201, 92, 22], OperandSize::Qword)
}

#[test]
fn vsubpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM8)), operand3: Some(IndirectScaledIndexed(RAX, RCX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 113, 189, 223, 92, 60, 200], OperandSize::Qword)
}

