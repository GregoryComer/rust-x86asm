use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpunpcklqdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 108, 255], OperandSize::Dword)
}

#[test]
fn vpunpcklqdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(EDI, 375153481, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 108, 143, 73, 99, 92, 22], OperandSize::Dword)
}

#[test]
fn vpunpcklqdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 108, 231], OperandSize::Qword)
}

#[test]
fn vpunpcklqdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 875331079, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 108, 52, 69, 7, 126, 44, 52], OperandSize::Qword)
}

#[test]
fn vpunpcklqdq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 108, 230], OperandSize::Dword)
}

#[test]
fn vpunpcklqdq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(EBX, 1211868827, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 108, 139, 155, 166, 59, 72], OperandSize::Dword)
}

#[test]
fn vpunpcklqdq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 108, 218], OperandSize::Qword)
}

#[test]
fn vpunpcklqdq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Two, 1833171305, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 108, 148, 83, 105, 249, 67, 109], OperandSize::Qword)
}

#[test]
fn vpunpcklqdq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 229, 137, 108, 199], OperandSize::Dword)
}

#[test]
fn vpunpcklqdq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(EDI, ECX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 213, 140, 108, 28, 207], OperandSize::Dword)
}

#[test]
fn vpunpcklqdq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(ESI, 127637169, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 229, 154, 108, 182, 177, 150, 155, 7], OperandSize::Dword)
}

#[test]
fn vpunpcklqdq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM28)), operand3: Some(Direct(XMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 17, 157, 134, 108, 234], OperandSize::Qword)
}

#[test]
fn vpunpcklqdq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM9)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Eight, 1698430905, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 181, 138, 108, 180, 192, 185, 255, 59, 101], OperandSize::Qword)
}

#[test]
fn vpunpcklqdq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM11)), operand3: Some(IndirectScaledIndexed(RBX, RSI, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 165, 158, 108, 44, 179], OperandSize::Qword)
}

