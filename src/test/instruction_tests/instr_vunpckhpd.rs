use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vunpckhpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 21, 232], OperandSize::Dword)
}

#[test]
fn vunpckhpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(EAX, 607192483, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 21, 184, 163, 5, 49, 36], OperandSize::Dword)
}

#[test]
fn vunpckhpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 21, 244], OperandSize::Qword)
}

#[test]
fn vunpckhpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(RCX, 1501622079, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 21, 145, 63, 239, 128, 89], OperandSize::Qword)
}

#[test]
fn vunpckhpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 21, 221], OperandSize::Dword)
}

#[test]
fn vunpckhpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 747499558, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 21, 36, 245, 38, 240, 141, 44], OperandSize::Dword)
}

#[test]
fn vunpckhpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 21, 202], OperandSize::Qword)
}

#[test]
fn vunpckhpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(RDI, 1166062314, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 21, 191, 234, 178, 128, 69], OperandSize::Qword)
}

#[test]
fn vunpckhpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 229, 138, 21, 196], OperandSize::Dword)
}

#[test]
fn vunpckhpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 885006790, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 197, 140, 21, 44, 117, 198, 33, 192, 52], OperandSize::Dword)
}

#[test]
fn vunpckhpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 197, 159, 21, 51], OperandSize::Dword)
}

#[test]
fn vunpckhpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM10)), operand3: Some(Direct(XMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 177, 173, 142, 21, 204], OperandSize::Qword)
}

#[test]
fn vunpckhpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM26)), operand3: Some(IndirectScaledIndexed(RBX, RAX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 173, 132, 21, 4, 195], OperandSize::Qword)
}

#[test]
fn vunpckhpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM26)), operand3: Some(IndirectScaledIndexed(RAX, RDX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 97, 173, 149, 21, 60, 80], OperandSize::Qword)
}

#[test]
fn vunpckhpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 221, 173, 21, 218], OperandSize::Dword)
}

#[test]
fn vunpckhpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 237, 169, 21, 39], OperandSize::Dword)
}

#[test]
fn vunpckhpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(ECX, ECX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 205, 187, 21, 4, 201], OperandSize::Dword)
}

#[test]
fn vunpckhpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM31)), operand3: Some(Direct(YMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 177, 133, 164, 21, 196], OperandSize::Qword)
}

#[test]
fn vunpckhpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 663516005, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 253, 173, 21, 60, 197, 101, 115, 140, 39], OperandSize::Qword)
}

#[test]
fn vunpckhpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 1498465737, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 225, 229, 188, 21, 36, 189, 201, 197, 80, 89], OperandSize::Qword)
}

#[test]
fn vunpckhpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 205, 201, 21, 216], OperandSize::Dword)
}

#[test]
fn vunpckhpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Four, 1770496343, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 245, 206, 21, 140, 129, 87, 161, 135, 105], OperandSize::Dword)
}

#[test]
fn vunpckhpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Four, 244884290, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 245, 219, 21, 180, 182, 66, 163, 152, 14], OperandSize::Dword)
}

#[test]
fn vunpckhpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM8)), operand3: Some(Direct(ZMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 1, 189, 203, 21, 242], OperandSize::Qword)
}

#[test]
fn vunpckhpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM16)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 648847468, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 253, 196, 21, 4, 213, 108, 160, 172, 38], OperandSize::Qword)
}

#[test]
fn vunpckhpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM19)), operand3: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 113, 229, 209, 21, 24], OperandSize::Qword)
}

