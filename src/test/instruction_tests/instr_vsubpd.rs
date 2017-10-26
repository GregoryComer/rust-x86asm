use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vsubpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 92, 204], OperandSize::Dword)
}

#[test]
fn vsubpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 92, 40], OperandSize::Dword)
}

#[test]
fn vsubpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 92, 234], OperandSize::Qword)
}

#[test]
fn vsubpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 806015875, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 92, 20, 77, 131, 211, 10, 48], OperandSize::Qword)
}

#[test]
fn vsubpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 92, 232], OperandSize::Dword)
}

#[test]
fn vsubpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 92, 9], OperandSize::Dword)
}

#[test]
fn vsubpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 92, 246], OperandSize::Qword)
}

#[test]
fn vsubpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(RSI, RCX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 92, 60, 142], OperandSize::Qword)
}

#[test]
fn vsubpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 197, 139, 92, 242], OperandSize::Dword)
}

#[test]
fn vsubpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Four, 874032050, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 213, 141, 92, 148, 183, 178, 171, 24, 52], OperandSize::Dword)
}

#[test]
fn vsubpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(EDX, EDX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 229, 155, 92, 60, 82], OperandSize::Dword)
}

#[test]
fn vsubpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM23)), operand3: Some(Direct(XMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 65, 197, 133, 92, 201], OperandSize::Qword)
}

#[test]
fn vsubpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM24)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 2015454678, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 189, 131, 92, 44, 141, 214, 101, 33, 120], OperandSize::Qword)
}

#[test]
fn vsubpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Four, 2124823742, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 213, 155, 92, 156, 144, 190, 60, 166, 126], OperandSize::Qword)
}

#[test]
fn vsubpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 174, 92, 243], OperandSize::Dword)
}

#[test]
fn vsubpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 1150314242, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 229, 175, 92, 4, 149, 2, 103, 144, 68], OperandSize::Dword)
}

#[test]
fn vsubpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(EDI, ECX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 197, 188, 92, 4, 79], OperandSize::Dword)
}

#[test]
fn vsubpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM8)), operand3: Some(Direct(YMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 33, 189, 172, 92, 231], OperandSize::Qword)
}

#[test]
fn vsubpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM31)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Eight, 395662465, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 133, 165, 92, 132, 206, 129, 84, 149, 23], OperandSize::Qword)
}

#[test]
fn vsubpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM11)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 874636748, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 113, 165, 191, 92, 4, 125, 204, 229, 33, 52], OperandSize::Qword)
}

#[test]
fn vsubpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 245, 253, 92, 248], OperandSize::Dword)
}

#[test]
fn vsubpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM4)), operand3: Some(Indirect(ECX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 221, 202, 92, 25], OperandSize::Dword)
}

#[test]
fn vsubpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectDisplaced(EAX, 1457560641, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 229, 217, 92, 176, 65, 156, 224, 86], OperandSize::Dword)
}

#[test]
fn vsubpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM11)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 209, 197, 255, 92, 203], OperandSize::Qword)
}

#[test]
fn vsubpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM14)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Eight, 1860339696, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 141, 205, 92, 172, 223, 240, 135, 226, 110], OperandSize::Qword)
}

#[test]
fn vsubpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM19)), operand3: Some(IndirectDisplaced(RBX, 222544316, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 113, 229, 215, 92, 179, 188, 193, 67, 13], OperandSize::Qword)
}

