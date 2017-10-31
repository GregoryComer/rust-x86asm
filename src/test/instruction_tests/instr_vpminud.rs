use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpminud_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 59, 250], OperandSize::Dword)
}

#[test]
fn vpminud_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 59, 47], OperandSize::Dword)
}

#[test]
fn vpminud_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 59, 219], OperandSize::Qword)
}

#[test]
fn vpminud_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(RSI, 870031701, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 59, 174, 85, 161, 219, 51], OperandSize::Qword)
}

#[test]
fn vpminud_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 59, 234], OperandSize::Dword)
}

#[test]
fn vpminud_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Four, 1039246845, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 59, 140, 135, 253, 165, 241, 61], OperandSize::Dword)
}

#[test]
fn vpminud_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 59, 252], OperandSize::Qword)
}

#[test]
fn vpminud_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 552723531, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 59, 12, 221, 75, 228, 241, 32], OperandSize::Qword)
}

#[test]
fn vpminud_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 69, 137, 59, 206], OperandSize::Dword)
}

#[test]
fn vpminud_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 69, 138, 59, 38], OperandSize::Dword)
}

#[test]
fn vpminud_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(EBX, EAX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 85, 155, 59, 28, 67], OperandSize::Dword)
}

#[test]
fn vpminud_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM18)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 109, 135, 59, 200], OperandSize::Qword)
}

#[test]
fn vpminud_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM16)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 2081257577, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 125, 134, 59, 60, 189, 105, 120, 13, 124], OperandSize::Qword)
}

#[test]
fn vpminud_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM10)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Four, 1706578700, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 45, 159, 59, 164, 182, 12, 83, 184, 101], OperandSize::Qword)
}

#[test]
fn vpminud_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 93, 174, 59, 200], OperandSize::Dword)
}

#[test]
fn vpminud_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 456353014, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 85, 173, 59, 20, 181, 246, 100, 51, 27], OperandSize::Dword)
}

#[test]
fn vpminud_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(ECX, 1592207984, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 85, 188, 59, 169, 112, 42, 231, 94], OperandSize::Dword)
}

#[test]
fn vpminud_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM30)), operand3: Some(Direct(YMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 194, 13, 165, 59, 253], OperandSize::Qword)
}

#[test]
fn vpminud_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM29)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Two, 1988964051, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 21, 162, 59, 156, 82, 211, 46, 141, 118], OperandSize::Qword)
}

#[test]
fn vpminud_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM17)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Eight, 171814814, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 117, 183, 59, 148, 247, 158, 175, 61, 10], OperandSize::Qword)
}

#[test]
fn vpminud_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 93, 203, 59, 214], OperandSize::Dword)
}

#[test]
fn vpminud_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Two, 930500053, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 69, 205, 59, 180, 121, 213, 77, 118, 55], OperandSize::Dword)
}

#[test]
fn vpminud_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectDisplaced(EAX, 2137818148, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 69, 217, 59, 128, 36, 132, 108, 127], OperandSize::Dword)
}

#[test]
fn vpminud_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 194, 109, 207, 59, 224], OperandSize::Qword)
}

#[test]
fn vpminud_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM16)), operand3: Some(IndirectScaledIndexed(RDI, RCX, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 199, 59, 60, 79], OperandSize::Qword)
}

#[test]
fn vpminud_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM12)), operand3: Some(IndirectDisplaced(RBX, 1819018533, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 29, 220, 59, 163, 37, 5, 108, 108], OperandSize::Qword)
}

