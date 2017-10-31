use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpunpckhqdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 109, 248], OperandSize::Dword)
}

#[test]
fn vpunpckhqdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(EAX, 1246946756, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 109, 176, 196, 229, 82, 74], OperandSize::Dword)
}

#[test]
fn vpunpckhqdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 109, 235], OperandSize::Qword)
}

#[test]
fn vpunpckhqdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(RSI, RBX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 109, 52, 222], OperandSize::Qword)
}

#[test]
fn vpunpckhqdq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 109, 205], OperandSize::Dword)
}

#[test]
fn vpunpckhqdq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(ESI, 1053825256, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 109, 182, 232, 24, 208, 62], OperandSize::Dword)
}

#[test]
fn vpunpckhqdq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 109, 236], OperandSize::Qword)
}

#[test]
fn vpunpckhqdq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(RCX, 1432956305, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 109, 153, 145, 45, 105, 85], OperandSize::Qword)
}

#[test]
fn vpunpckhqdq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 137, 109, 193], OperandSize::Dword)
}

#[test]
fn vpunpckhqdq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 999928644, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 253, 141, 109, 4, 181, 68, 179, 153, 59], OperandSize::Dword)
}

#[test]
fn vpunpckhqdq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EBX, EBX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 221, 158, 109, 44, 155], OperandSize::Dword)
}

#[test]
fn vpunpckhqdq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 1, 245, 143, 109, 217], OperandSize::Qword)
}

#[test]
fn vpunpckhqdq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM30)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 203499968, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 141, 131, 109, 20, 85, 192, 41, 33, 12], OperandSize::Qword)
}

#[test]
fn vpunpckhqdq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM18)), operand3: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 237, 148, 109, 50], OperandSize::Qword)
}

