use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vorps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 232, 86, 195], OperandSize::Dword)
}

#[test]
fn vorps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(EAX, EBX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 200, 86, 28, 88], OperandSize::Dword)
}

#[test]
fn vorps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 240, 86, 193], OperandSize::Qword)
}

#[test]
fn vorps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 240, 86, 23], OperandSize::Qword)
}

#[test]
fn vorps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 196, 86, 225], OperandSize::Dword)
}

#[test]
fn vorps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(EDX, 1693048360, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 244, 86, 170, 40, 222, 233, 100], OperandSize::Dword)
}

#[test]
fn vorps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 204, 86, 213], OperandSize::Qword)
}

#[test]
fn vorps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 1306317154, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 86, 20, 93, 98, 209, 220, 77], OperandSize::Qword)
}

#[test]
fn vorps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 68, 139, 86, 255], OperandSize::Dword)
}

#[test]
fn vorps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Four, 327528085, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 100, 141, 86, 156, 159, 149, 174, 133, 19], OperandSize::Dword)
}

#[test]
fn vorps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(ECX, EDI, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 68, 157, 86, 60, 185], OperandSize::Dword)
}

#[test]
fn vorps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM31)), operand3: Some(Direct(XMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 17, 4, 132, 86, 213], OperandSize::Qword)
}

#[test]
fn vorps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM21)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Four, 326798937, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 84, 133, 86, 164, 147, 89, 142, 122, 19], OperandSize::Qword)
}

#[test]
fn vorps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(RSI, 2091470288, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 113, 116, 159, 86, 142, 208, 77, 169, 124], OperandSize::Qword)
}

#[test]
fn vorps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 116, 173, 86, 239], OperandSize::Dword)
}

#[test]
fn vorps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(EDX, 679083814, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 92, 172, 86, 170, 38, 255, 121, 40], OperandSize::Dword)
}

#[test]
fn vorps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(ESI, ECX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 76, 188, 86, 36, 142], OperandSize::Dword)
}

#[test]
fn vorps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM29)), operand3: Some(Direct(YMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 177, 20, 162, 86, 252], OperandSize::Qword)
}

#[test]
fn vorps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM17)), operand3: Some(IndirectDisplaced(RDX, 1845906930, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 116, 166, 86, 162, 242, 77, 6, 110], OperandSize::Qword)
}

#[test]
fn vorps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM13)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Eight, 426770535, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 20, 190, 86, 156, 194, 103, 0, 112, 25], OperandSize::Qword)
}

#[test]
fn vorps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 84, 203, 86, 223], OperandSize::Dword)
}

#[test]
fn vorps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectDisplaced(ECX, 1414948897, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 108, 206, 86, 153, 33, 104, 86, 84], OperandSize::Dword)
}

#[test]
fn vorps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectDisplaced(EBX, 467545908, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 76, 219, 86, 187, 52, 47, 222, 27], OperandSize::Dword)
}

#[test]
fn vorps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM25)), operand3: Some(Direct(ZMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 81, 52, 197, 86, 247], OperandSize::Qword)
}

#[test]
fn vorps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM27)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 735082412, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 36, 197, 86, 12, 221, 172, 119, 208, 43], OperandSize::Qword)
}

#[test]
fn vorps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM11)), operand3: Some(IndirectScaledIndexed(RCX, RDX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 225, 36, 217, 86, 12, 81], OperandSize::Qword)
}

