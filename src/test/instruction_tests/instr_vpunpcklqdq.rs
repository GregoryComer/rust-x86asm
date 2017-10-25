use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpunpcklqdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 108, 210], OperandSize::Dword)
}

#[test]
fn vpunpcklqdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(ESI, 1037559409, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 108, 182, 113, 230, 215, 61], OperandSize::Dword)
}

#[test]
fn vpunpcklqdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 108, 255], OperandSize::Qword)
}

#[test]
fn vpunpcklqdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(RAX, RBX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 108, 60, 88], OperandSize::Qword)
}

#[test]
fn vpunpcklqdq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 108, 206], OperandSize::Dword)
}

#[test]
fn vpunpcklqdq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Four, 1131922829, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 108, 188, 159, 141, 197, 119, 67], OperandSize::Dword)
}

#[test]
fn vpunpcklqdq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 108, 233], OperandSize::Qword)
}

#[test]
fn vpunpcklqdq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 108, 14], OperandSize::Qword)
}

#[test]
fn vpunpcklqdq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 221, 143, 108, 240], OperandSize::Dword)
}

#[test]
fn vpunpcklqdq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(EBX, 164521998, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 197, 143, 108, 163, 14, 104, 206, 9], OperandSize::Dword)
}

#[test]
fn vpunpcklqdq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(ECX, 1881903881, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 213, 159, 108, 161, 9, 147, 43, 112], OperandSize::Dword)
}

#[test]
fn vpunpcklqdq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM18)), operand3: Some(Direct(XMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 177, 237, 134, 108, 193], OperandSize::Qword)
}

#[test]
fn vpunpcklqdq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM13)), operand3: Some(IndirectDisplaced(RCX, 1886181627, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 149, 141, 108, 169, 251, 216, 108, 112], OperandSize::Qword)
}

#[test]
fn vpunpcklqdq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM26)), operand3: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 225, 173, 151, 108, 16], OperandSize::Qword)
}

