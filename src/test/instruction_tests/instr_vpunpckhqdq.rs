use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpunpckhqdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 109, 228], OperandSize::Dword)
}

#[test]
fn vpunpckhqdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Four, 301961543, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 109, 148, 139, 71, 145, 255, 17], OperandSize::Dword)
}

#[test]
fn vpunpckhqdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 109, 212], OperandSize::Qword)
}

#[test]
fn vpunpckhqdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 1465334041, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 109, 20, 77, 25, 57, 87, 87], OperandSize::Qword)
}

#[test]
fn vpunpckhqdq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 109, 195], OperandSize::Dword)
}

#[test]
fn vpunpckhqdq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(EDI, 34871897, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 109, 151, 89, 26, 20, 2], OperandSize::Dword)
}

#[test]
fn vpunpckhqdq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 109, 246], OperandSize::Qword)
}

#[test]
fn vpunpckhqdq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Eight, 542425708, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 109, 148, 193, 108, 194, 84, 32], OperandSize::Qword)
}

#[test]
fn vpunpckhqdq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 137, 109, 216], OperandSize::Dword)
}

#[test]
fn vpunpckhqdq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 229, 143, 109, 14], OperandSize::Dword)
}

#[test]
fn vpunpckhqdq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(EDI, 1577459852, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 221, 158, 109, 159, 140, 32, 6, 94], OperandSize::Dword)
}

#[test]
fn vpunpckhqdq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM8)), operand3: Some(Direct(XMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 49, 189, 137, 109, 246], OperandSize::Qword)
}

#[test]
fn vpunpckhqdq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM13)), operand3: Some(IndirectScaledIndexed(RCX, RAX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 149, 139, 109, 20, 65], OperandSize::Qword)
}

#[test]
fn vpunpckhqdq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(RDX, 1305172221, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 225, 229, 159, 109, 154, 253, 88, 203, 77], OperandSize::Qword)
}

