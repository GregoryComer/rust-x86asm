use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vsqrtps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 81, 224], OperandSize::Dword)
}

#[test]
fn vsqrtps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(ESI, 335924013, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 81, 174, 45, 203, 5, 20], OperandSize::Dword)
}

#[test]
fn vsqrtps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 81, 230], OperandSize::Qword)
}

#[test]
fn vsqrtps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(RDI, Four, 1402327426, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 81, 28, 189, 130, 209, 149, 83], OperandSize::Qword)
}

#[test]
fn vsqrtps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 81, 227], OperandSize::Dword)
}

#[test]
fn vsqrtps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledDisplaced(ECX, Four, 297578723, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 81, 44, 141, 227, 176, 188, 17], OperandSize::Dword)
}

#[test]
fn vsqrtps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 81, 239], OperandSize::Qword)
}

#[test]
fn vsqrtps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(YMM2)), operand2: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 81, 19], OperandSize::Qword)
}

#[test]
fn vsqrtps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 124, 142, 81, 198], OperandSize::Dword)
}

#[test]
fn vsqrtps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Four, 1379031617, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 124, 142, 81, 148, 187, 65, 90, 50, 82], OperandSize::Dword)
}

#[test]
fn vsqrtps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(EBX, 1713476493, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 124, 153, 81, 179, 141, 147, 33, 102], OperandSize::Dword)
}

#[test]
fn vsqrtps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 124, 139, 81, 207], OperandSize::Qword)
}

#[test]
fn vsqrtps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(XMM15)), operand2: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 124, 143, 81, 59], OperandSize::Qword)
}

#[test]
fn vsqrtps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(XMM11)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Two, 1860731052, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 113, 124, 153, 81, 156, 65, 172, 128, 232, 110], OperandSize::Qword)
}

#[test]
fn vsqrtps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 124, 170, 81, 232], OperandSize::Dword)
}

#[test]
fn vsqrtps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(YMM7)), operand2: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 124, 174, 81, 56], OperandSize::Dword)
}

#[test]
fn vsqrtps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledIndexed(ESI, EDI, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 124, 190, 81, 52, 126], OperandSize::Dword)
}

#[test]
fn vsqrtps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 193, 124, 174, 81, 228], OperandSize::Qword)
}

#[test]
fn vsqrtps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(YMM26)), operand2: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 124, 174, 81, 18], OperandSize::Qword)
}

#[test]
fn vsqrtps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(YMM8)), operand2: Some(IndirectDisplaced(RCX, 1658445276, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 113, 124, 190, 81, 129, 220, 221, 217, 98], OperandSize::Qword)
}

#[test]
fn vsqrtps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 124, 255, 81, 236], OperandSize::Dword)
}

#[test]
fn vsqrtps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 136409207, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 124, 206, 81, 60, 69, 119, 112, 33, 8], OperandSize::Dword)
}

#[test]
fn vsqrtps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectDisplaced(ECX, 1479224596, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 124, 222, 81, 153, 20, 45, 43, 88], OperandSize::Dword)
}

#[test]
fn vsqrtps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM25)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 145, 124, 218, 81, 193], OperandSize::Qword)
}

#[test]
fn vsqrtps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(ZMM6)), operand2: Some(Indirect(RBX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 124, 201, 81, 51], OperandSize::Qword)
}

#[test]
fn vsqrtps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(ZMM24)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Two, 1204741252, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 97, 124, 221, 81, 132, 88, 132, 228, 206, 71], OperandSize::Qword)
}

