use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpunpckldq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 98, 235], OperandSize::Dword)
}

#[test]
fn vpunpckldq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Four, 1956870718, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 98, 188, 183, 62, 122, 163, 116], OperandSize::Dword)
}

#[test]
fn vpunpckldq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 98, 199], OperandSize::Qword)
}

#[test]
fn vpunpckldq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(RCX, RDX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 98, 12, 209], OperandSize::Qword)
}

#[test]
fn vpunpckldq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 98, 223], OperandSize::Dword)
}

#[test]
fn vpunpckldq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Eight, 1136567372, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 98, 180, 203, 76, 164, 190, 67], OperandSize::Dword)
}

#[test]
fn vpunpckldq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 98, 250], OperandSize::Qword)
}

#[test]
fn vpunpckldq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 2082548884, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 98, 4, 77, 148, 44, 33, 124], OperandSize::Qword)
}

#[test]
fn vpunpckldq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 93, 142, 98, 194], OperandSize::Dword)
}

#[test]
fn vpunpckldq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(EDX, 452752637, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 109, 143, 98, 178, 253, 116, 252, 26], OperandSize::Dword)
}

#[test]
fn vpunpckldq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 109, 154, 98, 25], OperandSize::Dword)
}

#[test]
fn vpunpckldq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM16)), operand3: Some(Direct(XMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 49, 125, 129, 98, 250], OperandSize::Qword)
}

#[test]
fn vpunpckldq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(RBX, RCX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 93, 143, 98, 44, 203], OperandSize::Qword)
}

#[test]
fn vpunpckldq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM20)), operand3: Some(IndirectDisplaced(RSI, 469600182, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 93, 145, 98, 158, 182, 135, 253, 27], OperandSize::Qword)
}

