use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpandnq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 229, 141, 223, 233], OperandSize::Dword)
}

#[test]
fn vpandnq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(ECX, 870068396, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 245, 137, 223, 137, 172, 48, 220, 51], OperandSize::Dword)
}

#[test]
fn vpandnq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(EDX, ECX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 205, 156, 223, 60, 74], OperandSize::Dword)
}

#[test]
fn vpandnq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM15)), operand3: Some(Direct(XMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 177, 133, 141, 223, 211], OperandSize::Qword)
}

#[test]
fn vpandnq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM14)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 141, 141, 223, 33], OperandSize::Qword)
}

#[test]
fn vpandnq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(RDI, 258115066, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 97, 197, 155, 223, 151, 250, 133, 98, 15], OperandSize::Qword)
}

#[test]
fn vpandnq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 175, 223, 247], OperandSize::Dword)
}

#[test]
fn vpandnq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 2145400446, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 213, 171, 223, 20, 221, 126, 54, 224, 127], OperandSize::Dword)
}

#[test]
fn vpandnq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 1325586550, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 229, 191, 223, 20, 85, 118, 216, 2, 79], OperandSize::Dword)
}

#[test]
fn vpandnq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM11)), operand3: Some(Direct(YMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 177, 165, 171, 223, 248], OperandSize::Qword)
}

#[test]
fn vpandnq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(RCX, RAX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 205, 175, 223, 36, 193], OperandSize::Qword)
}

#[test]
fn vpandnq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM22)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 1215659993, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 225, 205, 178, 223, 52, 213, 217, 127, 117, 72], OperandSize::Qword)
}

#[test]
fn vpandnq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 237, 205, 223, 249], OperandSize::Dword)
}

#[test]
fn vpandnq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 794518316, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 205, 203, 223, 4, 141, 44, 99, 91, 47], OperandSize::Dword)
}

#[test]
fn vpandnq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexed(EDX, ECX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 213, 219, 223, 60, 202], OperandSize::Dword)
}

#[test]
fn vpandnq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM21)), operand3: Some(Direct(ZMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 33, 213, 193, 223, 200], OperandSize::Qword)
}

#[test]
fn vpandnq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM8)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Four, 519164133, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 189, 207, 223, 188, 137, 229, 208, 241, 30], OperandSize::Qword)
}

#[test]
fn vpandnq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM27)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 1579083147, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 165, 214, 223, 20, 205, 139, 229, 30, 94], OperandSize::Qword)
}

