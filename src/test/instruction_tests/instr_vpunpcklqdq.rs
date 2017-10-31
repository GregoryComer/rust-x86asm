use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpunpcklqdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 108, 209], OperandSize::Dword)
}

#[test]
fn vpunpcklqdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(EDI, 1807989675, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 108, 191, 171, 187, 195, 107], OperandSize::Dword)
}

#[test]
fn vpunpcklqdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 108, 232], OperandSize::Qword)
}

#[test]
fn vpunpcklqdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(RSI, 270859935, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 108, 142, 159, 254, 36, 16], OperandSize::Qword)
}

#[test]
fn vpunpcklqdq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 108, 202], OperandSize::Dword)
}

#[test]
fn vpunpcklqdq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 1751600601, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 108, 4, 213, 217, 77, 103, 104], OperandSize::Dword)
}

#[test]
fn vpunpcklqdq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 108, 192], OperandSize::Qword)
}

#[test]
fn vpunpcklqdq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 108, 6], OperandSize::Qword)
}

#[test]
fn vpunpcklqdq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 229, 142, 108, 241], OperandSize::Dword)
}

#[test]
fn vpunpcklqdq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Four, 2146881573, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 237, 138, 108, 148, 131, 37, 208, 246, 127], OperandSize::Dword)
}

#[test]
fn vpunpcklqdq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 1461993378, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 253, 155, 108, 4, 221, 162, 63, 36, 87], OperandSize::Dword)
}

#[test]
fn vpunpcklqdq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM31)), operand3: Some(Direct(XMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 145, 133, 134, 108, 255], OperandSize::Qword)
}

#[test]
fn vpunpcklqdq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(RCX, 682991428, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 237, 142, 108, 185, 68, 159, 181, 40], OperandSize::Qword)
}

#[test]
fn vpunpcklqdq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM30)), operand3: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 97, 141, 147, 108, 22], OperandSize::Qword)
}

