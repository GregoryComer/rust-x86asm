use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vorps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 224, 86, 220], OperandSize::Dword)
}

#[test]
fn vorps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(ESI, 889216265, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 224, 86, 150, 9, 93, 0, 53], OperandSize::Dword)
}

#[test]
fn vorps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 224, 86, 246], OperandSize::Qword)
}

#[test]
fn vorps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 208, 86, 33], OperandSize::Qword)
}

#[test]
fn vorps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 220, 86, 220], OperandSize::Dword)
}

#[test]
fn vorps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 244, 86, 9], OperandSize::Dword)
}

#[test]
fn vorps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 244, 86, 249], OperandSize::Qword)
}

#[test]
fn vorps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 212, 86, 51], OperandSize::Qword)
}

#[test]
fn vorps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 100, 141, 86, 211], OperandSize::Dword)
}

#[test]
fn vorps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(EDX, ESI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 124, 138, 86, 36, 242], OperandSize::Dword)
}

#[test]
fn vorps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Eight, 1873629249, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 84, 158, 86, 172, 241, 65, 80, 173, 111], OperandSize::Dword)
}

#[test]
fn vorps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM19)), operand3: Some(Direct(XMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 145, 100, 135, 86, 201], OperandSize::Qword)
}

#[test]
fn vorps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Four, 617255097, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 124, 141, 86, 132, 185, 185, 144, 202, 36], OperandSize::Qword)
}

#[test]
fn vorps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM14)), operand3: Some(IndirectScaledIndexed(RCX, RBX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 225, 12, 158, 86, 4, 89], OperandSize::Qword)
}

#[test]
fn vorps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 84, 173, 86, 250], OperandSize::Dword)
}

#[test]
fn vorps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 92, 175, 86, 30], OperandSize::Dword)
}

#[test]
fn vorps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Two, 800553884, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 68, 186, 86, 180, 64, 156, 123, 183, 47], OperandSize::Dword)
}

#[test]
fn vorps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 92, 172, 86, 215], OperandSize::Qword)
}

#[test]
fn vorps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM15)), operand3: Some(IndirectScaledIndexed(RDI, RSI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 4, 173, 86, 20, 183], OperandSize::Qword)
}

#[test]
fn vorps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM12)), operand3: Some(IndirectDisplaced(RSI, 1001991028, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 225, 28, 188, 86, 166, 116, 43, 185, 59], OperandSize::Qword)
}

#[test]
fn vorps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 100, 202, 86, 226], OperandSize::Dword)
}

#[test]
fn vorps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 491508089, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 116, 204, 86, 36, 117, 121, 209, 75, 29], OperandSize::Dword)
}

#[test]
fn vorps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 1990240523, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 100, 222, 86, 28, 117, 11, 169, 160, 118], OperandSize::Dword)
}

#[test]
fn vorps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM9)), operand3: Some(Direct(ZMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 145, 52, 204, 86, 197], OperandSize::Qword)
}

#[test]
fn vorps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Four, 1625069545, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 92, 204, 86, 148, 134, 233, 151, 220, 96], OperandSize::Qword)
}

#[test]
fn vorps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM24)), operand3: Some(IndirectDisplaced(RSI, 2040668535, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 225, 60, 213, 86, 174, 119, 33, 162, 121], OperandSize::Qword)
}

