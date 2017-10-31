use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmulps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 224, 89, 219], OperandSize::Dword)
}

#[test]
fn vmulps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Two, 936924264, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 240, 89, 156, 122, 104, 84, 216, 55], OperandSize::Dword)
}

#[test]
fn vmulps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 200, 89, 209], OperandSize::Qword)
}

#[test]
fn vmulps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Four, 601077513, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 240, 89, 148, 190, 9, 183, 211, 35], OperandSize::Qword)
}

#[test]
fn vmulps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 212, 89, 240], OperandSize::Dword)
}

#[test]
fn vmulps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(EBX, ECX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 196, 89, 36, 75], OperandSize::Dword)
}

#[test]
fn vmulps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 228, 89, 251], OperandSize::Qword)
}

#[test]
fn vmulps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Eight, 630464635, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 244, 89, 140, 202, 123, 32, 148, 37], OperandSize::Qword)
}

#[test]
fn vmulps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 84, 138, 89, 239], OperandSize::Dword)
}

#[test]
fn vmulps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 1545832853, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 100, 140, 89, 12, 245, 149, 137, 35, 92], OperandSize::Dword)
}

#[test]
fn vmulps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 1877442738, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 108, 153, 89, 12, 77, 178, 128, 231, 111], OperandSize::Dword)
}

#[test]
fn vmulps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM30)), operand3: Some(Direct(XMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 193, 12, 134, 89, 238], OperandSize::Qword)
}

#[test]
fn vmulps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM29)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Two, 97077667, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 20, 131, 89, 172, 79, 163, 73, 201, 5], OperandSize::Qword)
}

#[test]
fn vmulps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(RDI, RBX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 84, 156, 89, 52, 95], OperandSize::Qword)
}

#[test]
fn vmulps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 76, 169, 89, 241], OperandSize::Dword)
}

#[test]
fn vmulps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(EBX, 221427824, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 92, 170, 89, 179, 112, 184, 50, 13], OperandSize::Dword)
}

#[test]
fn vmulps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Two, 496216659, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 76, 186, 89, 164, 82, 83, 170, 147, 29], OperandSize::Dword)
}

#[test]
fn vmulps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM17)), operand3: Some(Direct(YMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 33, 116, 166, 89, 251], OperandSize::Qword)
}

#[test]
fn vmulps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM10)), operand3: Some(IndirectScaledIndexed(RDX, RDX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 44, 172, 89, 12, 82], OperandSize::Qword)
}

#[test]
fn vmulps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM24)), operand3: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 113, 60, 183, 89, 35], OperandSize::Qword)
}

#[test]
fn vmulps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 108, 190, 89, 228], OperandSize::Dword)
}

#[test]
fn vmulps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexed(ESI, ESI, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 100, 206, 89, 52, 182], OperandSize::Dword)
}

#[test]
fn vmulps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexed(ECX, ESI, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 68, 220, 89, 52, 177], OperandSize::Dword)
}

#[test]
fn vmulps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM23)), operand3: Some(Direct(ZMM20)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 49, 68, 147, 89, 220], OperandSize::Qword)
}

#[test]
fn vmulps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM27)), operand3: Some(Indirect(RDI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 36, 195, 89, 15], OperandSize::Qword)
}

#[test]
fn vmulps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM30)), operand3: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 97, 12, 211, 89, 19], OperandSize::Qword)
}

