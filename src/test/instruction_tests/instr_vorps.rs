use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vorps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 232, 86, 239], OperandSize::Dword)
}

#[test]
fn vorps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 224, 86, 63], OperandSize::Dword)
}

#[test]
fn vorps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 224, 86, 208], OperandSize::Qword)
}

#[test]
fn vorps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Four, 1501342428, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 232, 86, 156, 129, 220, 170, 124, 89], OperandSize::Qword)
}

#[test]
fn vorps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 236, 86, 200], OperandSize::Dword)
}

#[test]
fn vorps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 441340441, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 244, 86, 12, 197, 25, 82, 78, 26], OperandSize::Dword)
}

#[test]
fn vorps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 212, 86, 200], OperandSize::Qword)
}

#[test]
fn vorps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(RBX, RDX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 244, 86, 12, 147], OperandSize::Qword)
}

#[test]
fn vorps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 84, 140, 86, 205], OperandSize::Dword)
}

#[test]
fn vorps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 712515346, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 84, 140, 86, 20, 149, 18, 31, 120, 42], OperandSize::Dword)
}

#[test]
fn vorps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Two, 75458616, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 108, 154, 86, 172, 81, 56, 104, 127, 4], OperandSize::Dword)
}

#[test]
fn vorps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM25)), operand3: Some(Direct(XMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 33, 52, 131, 86, 215], OperandSize::Qword)
}

#[test]
fn vorps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM24)), operand3: Some(IndirectScaledIndexed(RDX, RDX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 60, 133, 86, 28, 210], OperandSize::Qword)
}

#[test]
fn vorps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM31)), operand3: Some(IndirectScaledIndexed(RBX, RDX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 225, 4, 146, 86, 28, 211], OperandSize::Qword)
}

#[test]
fn vorps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 116, 170, 86, 232], OperandSize::Dword)
}

#[test]
fn vorps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 1037166893, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 68, 169, 86, 44, 181, 45, 233, 209, 61], OperandSize::Dword)
}

#[test]
fn vorps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 255129281, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 100, 191, 86, 12, 181, 193, 246, 52, 15], OperandSize::Dword)
}

#[test]
fn vorps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM10)), operand3: Some(Direct(YMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 65, 44, 172, 86, 198], OperandSize::Qword)
}

#[test]
fn vorps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM20)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Four, 1371803035, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 92, 166, 86, 140, 191, 155, 13, 196, 81], OperandSize::Qword)
}

#[test]
fn vorps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM11)), operand3: Some(IndirectScaledIndexed(RSI, RDX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 225, 36, 186, 86, 60, 86], OperandSize::Qword)
}

#[test]
fn vorps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 84, 203, 86, 239], OperandSize::Dword)
}

#[test]
fn vorps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(Indirect(EBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 76, 207, 86, 51], OperandSize::Dword)
}

#[test]
fn vorps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectDisplaced(EBX, 1464785279, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 84, 220, 86, 171, 127, 217, 78, 87], OperandSize::Dword)
}

#[test]
fn vorps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM10)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 44, 203, 86, 217], OperandSize::Qword)
}

#[test]
fn vorps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM28)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 1874074095, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 28, 198, 86, 20, 253, 239, 25, 180, 111], OperandSize::Qword)
}

#[test]
fn vorps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM19)), operand3: Some(IndirectDisplaced(RDX, 791398714, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 113, 100, 211, 86, 138, 58, 201, 43, 47], OperandSize::Qword)
}

