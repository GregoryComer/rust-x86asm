use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vdivps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 94, 227], OperandSize::Dword)
}

#[test]
fn vdivps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 192, 94, 31], OperandSize::Dword)
}

#[test]
fn vdivps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 208, 94, 210], OperandSize::Qword)
}

#[test]
fn vdivps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 216, 94, 56], OperandSize::Qword)
}

#[test]
fn vdivps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 228, 94, 200], OperandSize::Dword)
}

#[test]
fn vdivps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 244, 94, 42], OperandSize::Dword)
}

#[test]
fn vdivps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 196, 94, 213], OperandSize::Qword)
}

#[test]
fn vdivps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 220, 94, 18], OperandSize::Qword)
}

#[test]
fn vdivps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 76, 137, 94, 226], OperandSize::Dword)
}

#[test]
fn vdivps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Eight, 938137804, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 92, 138, 94, 172, 208, 204, 216, 234, 55], OperandSize::Dword)
}

#[test]
fn vdivps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(EDX, 14635626, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 68, 158, 94, 162, 106, 82, 223, 0], OperandSize::Dword)
}

#[test]
fn vdivps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM25)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 52, 135, 94, 248], OperandSize::Qword)
}

#[test]
fn vdivps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Two, 2059630241, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 4, 140, 94, 180, 66, 161, 118, 195, 122], OperandSize::Qword)
}

#[test]
fn vdivps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 1576073843, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 4, 157, 94, 36, 181, 115, 250, 240, 93], OperandSize::Qword)
}

#[test]
fn vdivps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 92, 174, 94, 211], OperandSize::Dword)
}

#[test]
fn vdivps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 1437882721, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 76, 170, 94, 36, 221, 97, 89, 180, 85], OperandSize::Dword)
}

#[test]
fn vdivps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(EDX, EDX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 76, 190, 94, 44, 210], OperandSize::Dword)
}

#[test]
fn vdivps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM11)), operand3: Some(Direct(YMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 129, 36, 172, 94, 235], OperandSize::Qword)
}

#[test]
fn vdivps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM17)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Four, 1537299385, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 116, 163, 94, 132, 131, 185, 83, 161, 91], OperandSize::Qword)
}

#[test]
fn vdivps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM20)), operand3: Some(IndirectDisplaced(RAX, 762297649, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 113, 92, 179, 94, 128, 49, 189, 111, 45], OperandSize::Qword)
}

#[test]
fn vdivps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 124, 255, 94, 254], OperandSize::Dword)
}

#[test]
fn vdivps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectDisplaced(EBX, 297605346, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 116, 205, 94, 171, 226, 24, 189, 17], OperandSize::Dword)
}

#[test]
fn vdivps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexed(ECX, ESI, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 68, 221, 94, 44, 177], OperandSize::Dword)
}

#[test]
fn vdivps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM9)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 52, 154, 94, 194], OperandSize::Qword)
}

#[test]
fn vdivps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM30)), operand3: Some(Indirect(RAX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 12, 196, 94, 24], OperandSize::Qword)
}

#[test]
fn vdivps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM17)), operand3: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 113, 116, 209, 94, 41], OperandSize::Qword)
}

