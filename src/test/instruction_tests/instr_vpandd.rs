use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpandd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 117, 141, 219, 244], OperandSize::Dword)
}

#[test]
fn vpandd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 2079789402, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 93, 143, 219, 36, 141, 90, 17, 247, 123], OperandSize::Dword)
}

#[test]
fn vpandd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Four, 975666265, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 117, 158, 219, 132, 135, 89, 124, 39, 58], OperandSize::Dword)
}

#[test]
fn vpandd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 33, 117, 138, 219, 203], OperandSize::Qword)
}

#[test]
fn vpandd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM11)), operand3: Some(IndirectScaledIndexed(RSI, RAX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 37, 139, 219, 44, 198], OperandSize::Qword)
}

#[test]
fn vpandd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM8)), operand3: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 97, 61, 159, 219, 27], OperandSize::Qword)
}

#[test]
fn vpandd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 77, 175, 219, 206], OperandSize::Dword)
}

#[test]
fn vpandd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(ESI, 1887138651, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 69, 172, 219, 158, 91, 115, 123, 112], OperandSize::Dword)
}

#[test]
fn vpandd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 101, 189, 219, 35], OperandSize::Dword)
}

#[test]
fn vpandd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM21)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 85, 164, 219, 252], OperandSize::Qword)
}

#[test]
fn vpandd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Two, 1021215916, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 101, 173, 219, 180, 118, 172, 132, 222, 60], OperandSize::Qword)
}

#[test]
fn vpandd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM15)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Eight, 1417251841, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 5, 188, 219, 188, 255, 1, 140, 121, 84], OperandSize::Qword)
}

#[test]
fn vpandd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 85, 206, 219, 232], OperandSize::Dword)
}

#[test]
fn vpandd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM4)), operand3: Some(Indirect(ESI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 93, 202, 219, 6], OperandSize::Dword)
}

#[test]
fn vpandd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM4)), operand3: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 93, 221, 219, 10], OperandSize::Dword)
}

#[test]
fn vpandd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM29)), operand3: Some(Direct(ZMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 17, 21, 197, 219, 241], OperandSize::Qword)
}

#[test]
fn vpandd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Two, 464030749, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 117, 205, 219, 172, 86, 29, 140, 168, 27], OperandSize::Qword)
}

#[test]
fn vpandd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM16)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Two, 2004219019, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 125, 215, 219, 156, 91, 139, 244, 117, 119], OperandSize::Qword)
}

