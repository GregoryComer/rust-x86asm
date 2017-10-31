use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmulps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 200, 89, 197], OperandSize::Dword)
}

#[test]
fn vmulps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 1895580662, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 216, 89, 36, 221, 246, 67, 252, 112], OperandSize::Dword)
}

#[test]
fn vmulps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 240, 89, 241], OperandSize::Qword)
}

#[test]
fn vmulps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(RSI, 1843223843, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 216, 89, 166, 35, 93, 221, 109], OperandSize::Qword)
}

#[test]
fn vmulps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 236, 89, 207], OperandSize::Dword)
}

#[test]
fn vmulps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(ECX, 155825496, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 236, 89, 137, 88, 181, 73, 9], OperandSize::Dword)
}

#[test]
fn vmulps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 236, 89, 200], OperandSize::Qword)
}

#[test]
fn vmulps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 2115663931, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 204, 89, 36, 157, 59, 120, 26, 126], OperandSize::Qword)
}

#[test]
fn vmulps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 108, 142, 89, 243], OperandSize::Dword)
}

#[test]
fn vmulps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(ESI, EAX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 100, 142, 89, 4, 134], OperandSize::Dword)
}

#[test]
fn vmulps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(ESI, EDX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 124, 159, 89, 4, 150], OperandSize::Dword)
}

#[test]
fn vmulps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM15)), operand3: Some(Direct(XMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 1, 4, 139, 89, 247], OperandSize::Qword)
}

#[test]
fn vmulps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Eight, 1022971215, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 92, 140, 89, 156, 250, 79, 77, 249, 60], OperandSize::Qword)
}

#[test]
fn vmulps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM24)), operand3: Some(IndirectDisplaced(RSI, 27461118, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 60, 146, 89, 182, 254, 5, 163, 1], OperandSize::Qword)
}

#[test]
fn vmulps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 108, 169, 89, 244], OperandSize::Dword)
}

#[test]
fn vmulps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(EBX, 1639207589, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 76, 173, 89, 163, 165, 82, 180, 97], OperandSize::Dword)
}

#[test]
fn vmulps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(EBX, 1991380389, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 76, 189, 89, 139, 165, 13, 178, 118], OperandSize::Dword)
}

#[test]
fn vmulps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM20)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 92, 167, 89, 220], OperandSize::Qword)
}

#[test]
fn vmulps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM13)), operand3: Some(IndirectScaledIndexed(RSI, RDX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 20, 175, 89, 44, 150], OperandSize::Qword)
}

#[test]
fn vmulps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(RDI, RCX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 97, 108, 185, 89, 28, 143], OperandSize::Qword)
}

#[test]
fn vmulps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 100, 185, 89, 245], OperandSize::Dword)
}

#[test]
fn vmulps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 1406620470, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 116, 205, 89, 20, 181, 54, 83, 215, 83], OperandSize::Dword)
}

#[test]
fn vmulps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 2075023651, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 100, 221, 89, 4, 85, 35, 89, 174, 123], OperandSize::Dword)
}

#[test]
fn vmulps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 100, 220, 89, 195], OperandSize::Qword)
}

#[test]
fn vmulps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM11)), operand3: Some(IndirectDisplaced(RBX, 9253263, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 36, 203, 89, 155, 143, 49, 141, 0], OperandSize::Qword)
}

#[test]
fn vmulps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM9)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Eight, 260767990, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 113, 52, 223, 89, 132, 247, 246, 0, 139, 15], OperandSize::Qword)
}

