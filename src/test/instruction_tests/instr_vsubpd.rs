use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vsubpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 92, 196], OperandSize::Dword)
}

#[test]
fn vsubpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(EDI, ESI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 92, 44, 247], OperandSize::Dword)
}

#[test]
fn vsubpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 92, 244], OperandSize::Qword)
}

#[test]
fn vsubpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Four, 1687190996, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 92, 180, 185, 212, 125, 144, 100], OperandSize::Qword)
}

#[test]
fn vsubpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 92, 230], OperandSize::Dword)
}

#[test]
fn vsubpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Two, 284751550, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 92, 188, 121, 190, 246, 248, 16], OperandSize::Dword)
}

#[test]
fn vsubpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 92, 241], OperandSize::Qword)
}

#[test]
fn vsubpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(RCX, 675956912, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 92, 145, 176, 72, 74, 40], OperandSize::Qword)
}

#[test]
fn vsubpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 197, 137, 92, 204], OperandSize::Dword)
}

#[test]
fn vsubpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(ECX, 1602777071, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 213, 141, 92, 129, 239, 111, 136, 95], OperandSize::Dword)
}

#[test]
fn vsubpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 205, 157, 92, 9], OperandSize::Dword)
}

#[test]
fn vsubpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM31)), operand3: Some(Direct(XMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 209, 133, 130, 92, 235], OperandSize::Qword)
}

#[test]
fn vsubpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM24)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 189, 135, 92, 0], OperandSize::Qword)
}

#[test]
fn vsubpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM8)), operand3: Some(IndirectScaledIndexed(RBX, RAX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 97, 189, 156, 92, 60, 131], OperandSize::Qword)
}

#[test]
fn vsubpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 229, 169, 92, 204], OperandSize::Dword)
}

#[test]
fn vsubpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Four, 167970227, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 221, 175, 92, 172, 185, 179, 5, 3, 10], OperandSize::Dword)
}

#[test]
fn vsubpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 245, 191, 92, 54], OperandSize::Dword)
}

#[test]
fn vsubpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 193, 237, 175, 92, 206], OperandSize::Qword)
}

#[test]
fn vsubpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM31)), operand3: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 133, 165, 92, 10], OperandSize::Qword)
}

#[test]
fn vsubpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM10)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 15351515, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 225, 173, 191, 92, 28, 149, 219, 62, 234, 0], OperandSize::Qword)
}

#[test]
fn vsubpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 237, 221, 92, 206], OperandSize::Dword)
}

#[test]
fn vsubpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectDisplaced(EAX, 953471931, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 205, 205, 92, 136, 187, 211, 212, 56], OperandSize::Dword)
}

#[test]
fn vsubpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 1264001766, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 245, 223, 92, 28, 133, 230, 34, 87, 75], OperandSize::Dword)
}

#[test]
fn vsubpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM14)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 209, 237, 221, 92, 246], OperandSize::Qword)
}

#[test]
fn vsubpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM25)), operand3: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 181, 193, 92, 10], OperandSize::Qword)
}

#[test]
fn vsubpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 872932304, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 113, 221, 220, 92, 52, 85, 208, 227, 7, 52], OperandSize::Qword)
}

