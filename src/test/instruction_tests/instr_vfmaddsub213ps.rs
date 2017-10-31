use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmaddsub213ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 166, 226], OperandSize::Dword)
}

#[test]
fn vfmaddsub213ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(ESI, 1923332135, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 166, 158, 39, 184, 163, 114], OperandSize::Dword)
}

#[test]
fn vfmaddsub213ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 166, 237], OperandSize::Qword)
}

#[test]
fn vfmaddsub213ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 1198981491, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 166, 20, 85, 115, 1, 119, 71], OperandSize::Qword)
}

#[test]
fn vfmaddsub213ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 166, 242], OperandSize::Dword)
}

#[test]
fn vfmaddsub213ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 1645778250, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 166, 36, 205, 74, 149, 24, 98], OperandSize::Dword)
}

#[test]
fn vfmaddsub213ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 166, 252], OperandSize::Qword)
}

#[test]
fn vfmaddsub213ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 808964989, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 166, 20, 253, 125, 211, 55, 48], OperandSize::Qword)
}

#[test]
fn vfmaddsub213ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 69, 143, 166, 247], OperandSize::Dword)
}

#[test]
fn vfmaddsub213ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 972862093, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 109, 141, 166, 4, 93, 141, 178, 252, 57], OperandSize::Dword)
}

#[test]
fn vfmaddsub213ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 1761597218, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 117, 156, 166, 20, 77, 34, 215, 255, 104], OperandSize::Dword)
}

#[test]
fn vfmaddsub213ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM22)), operand3: Some(Direct(XMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 162, 77, 132, 166, 197], OperandSize::Qword)
}

#[test]
fn vfmaddsub213ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM14)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Eight, 454048627, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 13, 139, 166, 188, 254, 115, 59, 16, 27], OperandSize::Qword)
}

#[test]
fn vfmaddsub213ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM14)), operand3: Some(IndirectScaledIndexed(RCX, RCX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 13, 157, 166, 20, 201], OperandSize::Qword)
}

#[test]
fn vfmaddsub213ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 77, 169, 166, 212], OperandSize::Dword)
}

#[test]
fn vfmaddsub213ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(EDI, EBX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 175, 166, 60, 95], OperandSize::Dword)
}

#[test]
fn vfmaddsub213ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 2134762102, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 77, 185, 166, 60, 149, 118, 226, 61, 127], OperandSize::Dword)
}

#[test]
fn vfmaddsub213ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 82, 125, 172, 166, 201], OperandSize::Qword)
}

#[test]
fn vfmaddsub213ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM13)), operand3: Some(IndirectScaledIndexed(RBX, RBX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 21, 175, 166, 4, 91], OperandSize::Qword)
}

#[test]
fn vfmaddsub213ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM26)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 192473841, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 45, 182, 166, 36, 77, 241, 234, 120, 11], OperandSize::Qword)
}

#[test]
fn vfmaddsub213ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 69, 185, 166, 240], OperandSize::Dword)
}

#[test]
fn vfmaddsub213ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectDisplaced(EDX, 161688239, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 101, 207, 166, 186, 175, 42, 163, 9], OperandSize::Dword)
}

#[test]
fn vfmaddsub213ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Eight, 296248708, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 101, 221, 166, 156, 249, 132, 101, 168, 17], OperandSize::Dword)
}

#[test]
fn vfmaddsub213ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM23)), operand3: Some(Direct(ZMM25)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 2, 69, 242, 166, 233], OperandSize::Qword)
}

#[test]
fn vfmaddsub213ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexed(RSI, RDX, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 101, 205, 166, 52, 86], OperandSize::Qword)
}

#[test]
fn vfmaddsub213ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM19)), operand3: Some(IndirectScaledIndexed(RCX, RDX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 226, 101, 211, 166, 60, 209], OperandSize::Qword)
}

