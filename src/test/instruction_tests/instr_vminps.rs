use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vminps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 224, 93, 235], OperandSize::Dword)
}

#[test]
fn vminps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 3790236, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 232, 93, 20, 117, 156, 213, 57, 0], OperandSize::Dword)
}

#[test]
fn vminps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 224, 93, 240], OperandSize::Qword)
}

#[test]
fn vminps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 93, 41], OperandSize::Qword)
}

#[test]
fn vminps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 228, 93, 216], OperandSize::Dword)
}

#[test]
fn vminps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 204, 93, 42], OperandSize::Dword)
}

#[test]
fn vminps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 236, 93, 202], OperandSize::Qword)
}

#[test]
fn vminps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 292689879, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 212, 93, 20, 125, 215, 23, 114, 17], OperandSize::Qword)
}

#[test]
fn vminps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 68, 142, 93, 255], OperandSize::Dword)
}

#[test]
fn vminps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(EDI, ECX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 108, 142, 93, 36, 143], OperandSize::Dword)
}

#[test]
fn vminps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Two, 1614504866, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 84, 158, 93, 132, 87, 162, 99, 59, 96], OperandSize::Dword)
}

#[test]
fn vminps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM21)), operand3: Some(Direct(XMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 17, 84, 134, 93, 218], OperandSize::Qword)
}

#[test]
fn vminps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM25)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 1299768001, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 52, 135, 93, 44, 133, 193, 226, 120, 77], OperandSize::Qword)
}

#[test]
fn vminps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM22)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 944493654, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 76, 151, 93, 44, 149, 86, 212, 75, 56], OperandSize::Qword)
}

#[test]
fn vminps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 124, 175, 93, 246], OperandSize::Dword)
}

#[test]
fn vminps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 1737638617, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 92, 172, 93, 44, 197, 217, 66, 146, 103], OperandSize::Dword)
}

#[test]
fn vminps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(EBX, 1900518954, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 116, 186, 93, 171, 42, 158, 71, 113], OperandSize::Dword)
}

#[test]
fn vminps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM8)), operand3: Some(Direct(YMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 129, 60, 171, 93, 235], OperandSize::Qword)
}

#[test]
fn vminps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM29)), operand3: Some(IndirectDisplaced(RBX, 2137165258, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 20, 162, 93, 147, 202, 141, 98, 127], OperandSize::Qword)
}

#[test]
fn vminps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM11)), operand3: Some(IndirectDisplaced(RAX, 980029685, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 113, 36, 188, 93, 160, 245, 16, 106, 58], OperandSize::Qword)
}

#[test]
fn vminps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 68, 159, 93, 253], OperandSize::Dword)
}

#[test]
fn vminps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectDisplaced(EAX, 1027552713, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 108, 202, 93, 176, 201, 53, 63, 61], OperandSize::Dword)
}

#[test]
fn vminps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM1)), operand3: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 116, 219, 93, 22], OperandSize::Dword)
}

#[test]
fn vminps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 161, 76, 158, 93, 214], OperandSize::Qword)
}

#[test]
fn vminps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM16)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Four, 1873244013, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 124, 193, 93, 140, 179, 109, 111, 167, 111], OperandSize::Qword)
}

#[test]
fn vminps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 1345048677, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 97, 76, 219, 93, 12, 93, 101, 208, 43, 80], OperandSize::Qword)
}

