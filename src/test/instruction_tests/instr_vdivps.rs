use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vdivps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 240, 94, 219], OperandSize::Dword)
}

#[test]
fn vdivps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(EBX, EBX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 232, 94, 12, 91], OperandSize::Dword)
}

#[test]
fn vdivps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 208, 94, 206], OperandSize::Qword)
}

#[test]
fn vdivps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 216, 94, 50], OperandSize::Qword)
}

#[test]
fn vdivps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 220, 94, 245], OperandSize::Dword)
}

#[test]
fn vdivps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 1795585498, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 204, 94, 28, 133, 218, 117, 6, 107], OperandSize::Dword)
}

#[test]
fn vdivps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 244, 94, 222], OperandSize::Qword)
}

#[test]
fn vdivps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 629196302, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 244, 94, 4, 253, 14, 198, 128, 37], OperandSize::Qword)
}

#[test]
fn vdivps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 124, 137, 94, 224], OperandSize::Dword)
}

#[test]
fn vdivps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 116, 139, 94, 41], OperandSize::Dword)
}

#[test]
fn vdivps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(EDI, 1875306033, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 100, 159, 94, 135, 49, 230, 198, 111], OperandSize::Dword)
}

#[test]
fn vdivps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM15)), operand3: Some(Direct(XMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 129, 4, 142, 94, 250], OperandSize::Qword)
}

#[test]
fn vdivps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM11)), operand3: Some(IndirectScaledIndexed(RAX, RBX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 36, 141, 94, 36, 216], OperandSize::Qword)
}

#[test]
fn vdivps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 1972755936, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 225, 92, 156, 94, 44, 125, 224, 221, 149, 117], OperandSize::Qword)
}

#[test]
fn vdivps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 92, 175, 94, 254], OperandSize::Dword)
}

#[test]
fn vdivps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(ESI, 1192777438, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 100, 172, 94, 134, 222, 86, 24, 71], OperandSize::Dword)
}

#[test]
fn vdivps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Eight, 84252113, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 108, 186, 94, 180, 211, 209, 149, 5, 5], OperandSize::Dword)
}

#[test]
fn vdivps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM18)), operand3: Some(Direct(YMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 193, 108, 163, 94, 228], OperandSize::Qword)
}

#[test]
fn vdivps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM29)), operand3: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 20, 162, 94, 62], OperandSize::Qword)
}

#[test]
fn vdivps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM18)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Eight, 1902833999, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 113, 108, 177, 94, 140, 200, 79, 241, 106, 113], OperandSize::Qword)
}

#[test]
fn vdivps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 92, 217, 94, 195], OperandSize::Dword)
}

#[test]
fn vdivps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectDisplaced(EBX, 305186185, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 124, 205, 94, 131, 137, 197, 48, 18], OperandSize::Dword)
}

#[test]
fn vdivps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Four, 1775951736, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 76, 221, 94, 132, 134, 120, 223, 218, 105], OperandSize::Dword)
}

#[test]
fn vdivps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM14)), operand3: Some(Direct(ZMM10)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 209, 12, 158, 94, 194], OperandSize::Qword)
}

#[test]
fn vdivps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM8)), operand3: Some(IndirectDisplaced(RDI, 172028450, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 60, 201, 94, 135, 34, 242, 64, 10], OperandSize::Qword)
}

#[test]
fn vdivps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM21)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Two, 1305270090, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 225, 84, 209, 94, 148, 123, 74, 215, 204, 77], OperandSize::Qword)
}

