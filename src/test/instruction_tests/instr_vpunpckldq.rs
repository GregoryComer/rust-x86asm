use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpunpckldq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 98, 245], OperandSize::Dword)
}

#[test]
fn vpunpckldq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EDX, ECX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 98, 12, 74], OperandSize::Dword)
}

#[test]
fn vpunpckldq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 98, 225], OperandSize::Qword)
}

#[test]
fn vpunpckldq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(RDX, 170997599, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 98, 186, 95, 55, 49, 10], OperandSize::Qword)
}

#[test]
fn vpunpckldq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 98, 209], OperandSize::Dword)
}

#[test]
fn vpunpckldq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Four, 1676860723, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 98, 140, 185, 51, 221, 242, 99], OperandSize::Dword)
}

#[test]
fn vpunpckldq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 98, 223], OperandSize::Qword)
}

#[test]
fn vpunpckldq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(RCX, RBX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 98, 28, 153], OperandSize::Qword)
}

#[test]
fn vpunpckldq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 143, 98, 213], OperandSize::Dword)
}

#[test]
fn vpunpckldq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Four, 1784019430, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 101, 140, 98, 164, 135, 230, 249, 85, 106], OperandSize::Dword)
}

#[test]
fn vpunpckldq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 85, 154, 98, 24], OperandSize::Dword)
}

#[test]
fn vpunpckldq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM25)), operand3: Some(Direct(XMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 81, 53, 129, 98, 253], OperandSize::Qword)
}

#[test]
fn vpunpckldq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 2005209702, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 69, 140, 98, 60, 93, 102, 18, 133, 119], OperandSize::Qword)
}

#[test]
fn vpunpckldq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM27)), operand3: Some(IndirectScaledIndexed(RBX, RBX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 37, 150, 98, 36, 219], OperandSize::Qword)
}

