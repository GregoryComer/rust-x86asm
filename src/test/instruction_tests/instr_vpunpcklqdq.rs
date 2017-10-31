use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpunpcklqdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 108, 250], OperandSize::Dword)
}

#[test]
fn vpunpcklqdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(ESI, EBX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 108, 12, 158], OperandSize::Dword)
}

#[test]
fn vpunpcklqdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 108, 252], OperandSize::Qword)
}

#[test]
fn vpunpcklqdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(RDX, RDX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 108, 60, 210], OperandSize::Qword)
}

#[test]
fn vpunpcklqdq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 108, 223], OperandSize::Dword)
}

#[test]
fn vpunpcklqdq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(EDX, EDI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 108, 20, 186], OperandSize::Dword)
}

#[test]
fn vpunpcklqdq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 108, 230], OperandSize::Qword)
}

#[test]
fn vpunpcklqdq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(RBX, 799583492, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 108, 139, 4, 173, 168, 47], OperandSize::Qword)
}

#[test]
fn vpunpcklqdq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 229, 137, 108, 248], OperandSize::Dword)
}

#[test]
fn vpunpcklqdq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 213, 139, 108, 22], OperandSize::Dword)
}

#[test]
fn vpunpcklqdq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 205, 154, 108, 34], OperandSize::Dword)
}

#[test]
fn vpunpcklqdq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 81, 229, 141, 108, 213], OperandSize::Qword)
}

#[test]
fn vpunpcklqdq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 1363804544, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 221, 142, 108, 28, 181, 128, 1, 74, 81], OperandSize::Qword)
}

#[test]
fn vpunpcklqdq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(RAX, 288976597, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 97, 221, 158, 108, 136, 213, 110, 57, 17], OperandSize::Qword)
}

