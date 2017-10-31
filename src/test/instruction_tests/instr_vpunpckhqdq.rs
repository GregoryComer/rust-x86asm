use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpunpckhqdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 109, 195], OperandSize::Dword)
}

#[test]
fn vpunpckhqdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 109, 26], OperandSize::Dword)
}

#[test]
fn vpunpckhqdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 109, 249], OperandSize::Qword)
}

#[test]
fn vpunpckhqdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Four, 2073687053, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 109, 172, 158, 13, 244, 153, 123], OperandSize::Qword)
}

#[test]
fn vpunpckhqdq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 109, 214], OperandSize::Dword)
}

#[test]
fn vpunpckhqdq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(EDX, EBX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 109, 28, 90], OperandSize::Dword)
}

#[test]
fn vpunpckhqdq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 109, 210], OperandSize::Qword)
}

#[test]
fn vpunpckhqdq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 1673020799, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 109, 20, 77, 127, 69, 184, 99], OperandSize::Qword)
}

#[test]
fn vpunpckhqdq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 197, 139, 109, 206], OperandSize::Dword)
}

#[test]
fn vpunpckhqdq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 923419611, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 237, 140, 109, 20, 149, 219, 67, 10, 55], OperandSize::Dword)
}

#[test]
fn vpunpckhqdq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(EBX, ECX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 205, 154, 109, 4, 75], OperandSize::Dword)
}

#[test]
fn vpunpckhqdq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 17, 205, 141, 109, 196], OperandSize::Qword)
}

#[test]
fn vpunpckhqdq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM29)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 149, 133, 109, 15], OperandSize::Qword)
}

#[test]
fn vpunpckhqdq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(RAX, RCX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 221, 153, 109, 36, 72], OperandSize::Qword)
}

