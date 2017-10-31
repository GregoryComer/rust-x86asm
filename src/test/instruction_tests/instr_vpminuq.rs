use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpminuq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 205, 143, 59, 228], OperandSize::Dword)
}

#[test]
fn vpminuq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Eight, 902653494, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 213, 142, 59, 148, 203, 54, 102, 205, 53], OperandSize::Dword)
}

#[test]
fn vpminuq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 221, 155, 59, 7], OperandSize::Dword)
}

#[test]
fn vpminuq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM22)), operand3: Some(Direct(XMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 146, 205, 129, 59, 195], OperandSize::Qword)
}

#[test]
fn vpminuq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM24)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 189, 135, 59, 19], OperandSize::Qword)
}

#[test]
fn vpminuq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(RBX, 1026266006, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 98, 245, 155, 59, 131, 150, 147, 43, 61], OperandSize::Qword)
}

#[test]
fn vpminuq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 197, 170, 59, 251], OperandSize::Dword)
}

#[test]
fn vpminuq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 1799533625, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 221, 174, 59, 12, 189, 57, 180, 66, 107], OperandSize::Dword)
}

#[test]
fn vpminuq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 442404241, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 221, 185, 59, 36, 221, 145, 141, 94, 26], OperandSize::Dword)
}

#[test]
fn vpminuq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM21)), operand3: Some(Direct(YMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 130, 213, 167, 59, 227], OperandSize::Qword)
}

#[test]
fn vpminuq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 1572977698, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 213, 171, 59, 52, 133, 34, 188, 193, 93], OperandSize::Qword)
}

#[test]
fn vpminuq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM21)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Eight, 1949712089, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 213, 182, 59, 148, 194, 217, 62, 54, 116], OperandSize::Qword)
}

#[test]
fn vpminuq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 221, 202, 59, 246], OperandSize::Dword)
}

#[test]
fn vpminuq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexed(EAX, EAX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 245, 203, 59, 12, 128], OperandSize::Dword)
}

#[test]
fn vpminuq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM6)), operand3: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 205, 217, 59, 34], OperandSize::Dword)
}

#[test]
fn vpminuq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM11)), operand3: Some(Direct(ZMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 146, 165, 207, 59, 212], OperandSize::Qword)
}

#[test]
fn vpminuq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM24)), operand3: Some(IndirectDisplaced(RDI, 475096780, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 189, 196, 59, 143, 204, 102, 81, 28], OperandSize::Qword)
}

#[test]
fn vpminuq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM30)), operand3: Some(IndirectScaledIndexed(RAX, RCX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 141, 211, 59, 28, 136], OperandSize::Qword)
}

