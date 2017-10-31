use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpandd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 101, 143, 219, 214], OperandSize::Dword)
}

#[test]
fn vpandd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 117, 142, 219, 24], OperandSize::Dword)
}

#[test]
fn vpandd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Two, 1355299599, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 69, 155, 219, 140, 95, 15, 59, 200, 80], OperandSize::Dword)
}

#[test]
fn vpandd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM21)), operand3: Some(Direct(XMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 49, 85, 132, 219, 216], OperandSize::Qword)
}

#[test]
fn vpandd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM19)), operand3: Some(IndirectDisplaced(RBX, 656815456, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 101, 134, 219, 139, 96, 53, 38, 39], OperandSize::Qword)
}

#[test]
fn vpandd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM30)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 1752588995, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 97, 13, 145, 219, 12, 77, 195, 98, 118, 104], OperandSize::Qword)
}

#[test]
fn vpandd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 77, 173, 219, 199], OperandSize::Dword)
}

#[test]
fn vpandd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(ECX, EDI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 85, 174, 219, 60, 121], OperandSize::Dword)
}

#[test]
fn vpandd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 109, 189, 219, 6], OperandSize::Dword)
}

#[test]
fn vpandd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 1, 69, 171, 219, 212], OperandSize::Qword)
}

#[test]
fn vpandd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(RCX, 1302790442, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 109, 170, 219, 153, 42, 1, 167, 77], OperandSize::Qword)
}

#[test]
fn vpandd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM22)), operand3: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 225, 77, 178, 219, 25], OperandSize::Qword)
}

#[test]
fn vpandd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 93, 204, 219, 221], OperandSize::Dword)
}

#[test]
fn vpandd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexed(ECX, ECX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 117, 205, 219, 36, 201], OperandSize::Dword)
}

#[test]
fn vpandd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectDisplaced(EDX, 1301326283, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 85, 220, 219, 178, 203, 169, 144, 77], OperandSize::Dword)
}

#[test]
fn vpandd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM27)), operand3: Some(Direct(ZMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 145, 37, 194, 219, 210], OperandSize::Qword)
}

#[test]
fn vpandd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM23)), operand3: Some(Indirect(RBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 69, 194, 219, 59], OperandSize::Qword)
}

#[test]
fn vpandd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM17)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 1740755117, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 225, 117, 214, 219, 20, 125, 173, 208, 193, 103], OperandSize::Qword)
}

