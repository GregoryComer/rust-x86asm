use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmulps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 232, 89, 235], OperandSize::Dword)
}

#[test]
fn vmulps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 224, 89, 16], OperandSize::Dword)
}

#[test]
fn vmulps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 89, 231], OperandSize::Qword)
}

#[test]
fn vmulps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 216, 89, 27], OperandSize::Qword)
}

#[test]
fn vmulps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 236, 89, 237], OperandSize::Dword)
}

#[test]
fn vmulps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(EDI, 1972191960, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 89, 151, 216, 66, 141, 117], OperandSize::Dword)
}

#[test]
fn vmulps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 212, 89, 241], OperandSize::Qword)
}

#[test]
fn vmulps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(RBX, 1303221437, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 89, 147, 189, 148, 173, 77], OperandSize::Qword)
}

#[test]
fn vmulps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 84, 138, 89, 235], OperandSize::Dword)
}

#[test]
fn vmulps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 1599921327, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 84, 140, 89, 20, 157, 175, 220, 92, 95], OperandSize::Dword)
}

#[test]
fn vmulps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EDX, 1876360161, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 76, 156, 89, 138, 225, 251, 214, 111], OperandSize::Dword)
}

#[test]
fn vmulps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM22)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 76, 130, 89, 194], OperandSize::Qword)
}

#[test]
fn vmulps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM15)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 4, 138, 89, 47], OperandSize::Qword)
}

#[test]
fn vmulps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(RCX, RAX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 113, 100, 157, 89, 12, 129], OperandSize::Qword)
}

#[test]
fn vmulps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 76, 173, 89, 196], OperandSize::Dword)
}

#[test]
fn vmulps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 1308165505, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 92, 169, 89, 36, 141, 129, 5, 249, 77], OperandSize::Dword)
}

#[test]
fn vmulps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(ESI, 640734378, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 100, 190, 89, 134, 170, 212, 48, 38], OperandSize::Dword)
}

#[test]
fn vmulps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM15)), operand3: Some(Direct(YMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 49, 4, 175, 89, 219], OperandSize::Qword)
}

#[test]
fn vmulps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM18)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 183363604, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 108, 163, 89, 44, 157, 20, 232, 237, 10], OperandSize::Qword)
}

#[test]
fn vmulps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Four, 1659138224, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 113, 108, 190, 89, 132, 158, 176, 112, 228, 98], OperandSize::Qword)
}

#[test]
fn vmulps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 100, 255, 89, 219], OperandSize::Dword)
}

#[test]
fn vmulps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectDisplaced(EDX, 500472194, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 116, 202, 89, 162, 130, 153, 212, 29], OperandSize::Dword)
}

#[test]
fn vmulps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexed(EDI, EBX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 84, 218, 89, 44, 95], OperandSize::Dword)
}

#[test]
fn vmulps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM24)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 129, 108, 155, 89, 192], OperandSize::Qword)
}

#[test]
fn vmulps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM31)), operand3: Some(IndirectScaledIndexed(RAX, RAX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 4, 199, 89, 12, 128], OperandSize::Qword)
}

#[test]
fn vmulps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectDisplaced(RBX, 213172279, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 100, 221, 89, 147, 55, 192, 180, 12], OperandSize::Qword)
}

