use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vdivps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 224, 94, 212], OperandSize::Dword)
}

#[test]
fn vdivps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Four, 119848022, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 192, 94, 156, 131, 86, 188, 36, 7], OperandSize::Dword)
}

#[test]
fn vdivps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 192, 94, 212], OperandSize::Qword)
}

#[test]
fn vdivps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 1550569473, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 94, 36, 197, 1, 208, 107, 92], OperandSize::Qword)
}

#[test]
fn vdivps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 196, 94, 226], OperandSize::Dword)
}

#[test]
fn vdivps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(EAX, EAX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 220, 94, 12, 128], OperandSize::Dword)
}

#[test]
fn vdivps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 196, 94, 255], OperandSize::Qword)
}

#[test]
fn vdivps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 429950054, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 212, 94, 52, 69, 102, 132, 160, 25], OperandSize::Qword)
}

#[test]
fn vdivps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 68, 141, 94, 233], OperandSize::Dword)
}

#[test]
fn vdivps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Eight, 632229996, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 92, 140, 94, 156, 255, 108, 16, 175, 37], OperandSize::Dword)
}

#[test]
fn vdivps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(EAX, 531307489, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 124, 158, 94, 176, 225, 27, 171, 31], OperandSize::Dword)
}

#[test]
fn vdivps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM15)), operand3: Some(Direct(XMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 193, 4, 140, 94, 211], OperandSize::Qword)
}

#[test]
fn vdivps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(RDI, 1925335361, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 108, 138, 94, 135, 65, 73, 194, 114], OperandSize::Qword)
}

#[test]
fn vdivps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM13)), operand3: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 113, 20, 157, 94, 6], OperandSize::Qword)
}

#[test]
fn vdivps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 100, 169, 94, 254], OperandSize::Dword)
}

#[test]
fn vdivps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(EDX, 134910740, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 100, 170, 94, 170, 20, 147, 10, 8], OperandSize::Dword)
}

#[test]
fn vdivps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(ESI, ESI, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 92, 186, 94, 28, 246], OperandSize::Dword)
}

#[test]
fn vdivps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM18)), operand3: Some(Direct(YMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 81, 108, 164, 94, 215], OperandSize::Qword)
}

#[test]
fn vdivps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM28)), operand3: Some(IndirectScaledIndexed(RCX, RDI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 28, 163, 94, 28, 185], OperandSize::Qword)
}

#[test]
fn vdivps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM16)), operand3: Some(IndirectDisplaced(RSI, 430405433, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 97, 124, 183, 94, 166, 57, 119, 167, 25], OperandSize::Qword)
}

#[test]
fn vdivps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 84, 249, 94, 214], OperandSize::Dword)
}

#[test]
fn vdivps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 899635930, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 84, 202, 94, 36, 189, 218, 90, 159, 53], OperandSize::Dword)
}

#[test]
fn vdivps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexed(EDI, ECX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 92, 223, 94, 36, 207], OperandSize::Dword)
}

#[test]
fn vdivps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM11)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 36, 250, 94, 244], OperandSize::Qword)
}

#[test]
fn vdivps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM26)), operand3: Some(IndirectScaledIndexed(RSI, RDI, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 44, 194, 94, 44, 254], OperandSize::Qword)
}

#[test]
fn vdivps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM24)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 1848483534, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 60, 215, 94, 28, 141, 206, 158, 45, 110], OperandSize::Qword)
}

