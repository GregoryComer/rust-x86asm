use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmaddsub231ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 182, 220], OperandSize::Dword)
}

#[test]
fn vfmaddsub231ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 182, 51], OperandSize::Dword)
}

#[test]
fn vfmaddsub231ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 182, 203], OperandSize::Qword)
}

#[test]
fn vfmaddsub231ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(RCX, 212179324, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 182, 145, 124, 153, 165, 12], OperandSize::Qword)
}

#[test]
fn vfmaddsub231ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 182, 230], OperandSize::Dword)
}

#[test]
fn vfmaddsub231ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Four, 412910539, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 182, 156, 158, 203, 131, 156, 24], OperandSize::Dword)
}

#[test]
fn vfmaddsub231ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 182, 240], OperandSize::Qword)
}

#[test]
fn vfmaddsub231ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Four, 487172269, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 182, 180, 130, 173, 168, 9, 29], OperandSize::Qword)
}

#[test]
fn vfmaddsub231ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 117, 138, 182, 226], OperandSize::Dword)
}

#[test]
fn vfmaddsub231ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Eight, 1410159855, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 77, 141, 182, 164, 248, 239, 84, 13, 84], OperandSize::Dword)
}

#[test]
fn vfmaddsub231ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 93, 156, 182, 35], OperandSize::Dword)
}

#[test]
fn vfmaddsub231ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 109, 138, 182, 246], OperandSize::Qword)
}

#[test]
fn vfmaddsub231ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 1883932477, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 85, 137, 182, 12, 77, 61, 135, 74, 112], OperandSize::Qword)
}

#[test]
fn vfmaddsub231ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM14)), operand3: Some(IndirectScaledIndexed(RDX, RBX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 13, 155, 182, 12, 154], OperandSize::Qword)
}

#[test]
fn vfmaddsub231ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 69, 174, 182, 248], OperandSize::Dword)
}

#[test]
fn vfmaddsub231ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(ESI, 698858208, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 172, 182, 174, 224, 186, 167, 41], OperandSize::Dword)
}

#[test]
fn vfmaddsub231ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(EBX, EDI, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 93, 189, 182, 52, 251], OperandSize::Dword)
}

#[test]
fn vfmaddsub231ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 85, 171, 182, 221], OperandSize::Qword)
}

#[test]
fn vfmaddsub231ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 77, 173, 182, 55], OperandSize::Qword)
}

#[test]
fn vfmaddsub231ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM21)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Four, 1536117988, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 85, 178, 182, 172, 159, 228, 76, 143, 91], OperandSize::Qword)
}

#[test]
fn vfmaddsub231ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 117, 254, 182, 192], OperandSize::Dword)
}

#[test]
fn vfmaddsub231ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 1164420321, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 85, 201, 182, 36, 125, 225, 164, 103, 69], OperandSize::Dword)
}

#[test]
fn vfmaddsub231ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Eight, 453978075, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 77, 219, 182, 172, 194, 219, 39, 15, 27], OperandSize::Dword)
}

#[test]
fn vfmaddsub231ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 85, 255, 182, 239], OperandSize::Qword)
}

#[test]
fn vfmaddsub231ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM27)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Four, 1052563379, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 37, 196, 182, 140, 138, 179, 215, 188, 62], OperandSize::Qword)
}

#[test]
fn vfmaddsub231ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM23)), operand3: Some(IndirectScaledIndexed(RAX, RCX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 69, 214, 182, 36, 72], OperandSize::Qword)
}

