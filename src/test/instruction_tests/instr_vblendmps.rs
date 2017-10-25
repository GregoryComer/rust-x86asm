use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vblendmps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 85, 142, 101, 246], OperandSize::Dword)
}

#[test]
fn vblendmps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(EDI, EDI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 101, 140, 101, 44, 127], OperandSize::Dword)
}

#[test]
fn vblendmps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(EDX, 1262792795, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 101, 154, 101, 186, 91, 176, 68, 75], OperandSize::Dword)
}

#[test]
fn vblendmps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM21)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 85, 129, 101, 230], OperandSize::Qword)
}

#[test]
fn vblendmps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 69, 138, 101, 9], OperandSize::Qword)
}

#[test]
fn vblendmps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM22)), operand3: Some(IndirectDisplaced(RDX, 1267538641, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 77, 150, 101, 138, 209, 26, 141, 75], OperandSize::Qword)
}

#[test]
fn vblendmps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 77, 171, 101, 221], OperandSize::Dword)
}

#[test]
fn vblendmps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(EDX, EDX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 85, 173, 101, 28, 146], OperandSize::Dword)
}

#[test]
fn vblendmps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 93, 189, 101, 35], OperandSize::Dword)
}

#[test]
fn vblendmps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM28)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 29, 165, 101, 231], OperandSize::Qword)
}

#[test]
fn vblendmps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM25)), operand3: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 53, 162, 101, 51], OperandSize::Qword)
}

#[test]
fn vblendmps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM13)), operand3: Some(IndirectScaledIndexed(RDX, RDX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 21, 189, 101, 28, 82], OperandSize::Qword)
}

#[test]
fn vblendmps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 77, 204, 101, 237], OperandSize::Dword)
}

#[test]
fn vblendmps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 453656995, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 109, 205, 101, 52, 181, 163, 65, 10, 27], OperandSize::Dword)
}

#[test]
fn vblendmps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM7)), operand3: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 69, 218, 101, 55], OperandSize::Dword)
}

#[test]
fn vblendmps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM28)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 29, 198, 101, 213], OperandSize::Qword)
}

#[test]
fn vblendmps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM29)), operand3: Some(IndirectDisplaced(RAX, 741031606, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 21, 197, 101, 144, 182, 62, 43, 44], OperandSize::Qword)
}

#[test]
fn vblendmps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM23)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Four, 1733389659, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 69, 210, 101, 148, 191, 91, 109, 81, 103], OperandSize::Qword)
}

