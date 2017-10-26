use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vprolq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(63)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 205, 138, 114, 202, 63], OperandSize::Dword)
}

#[test]
fn vprolq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(ESI, ESI, Four, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(108)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 245, 139, 114, 12, 182, 108], OperandSize::Dword)
}

#[test]
fn vprolq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand3: Some(Literal8(13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 221, 156, 114, 10, 13], OperandSize::Dword)
}

#[test]
fn vprolq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(71)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 173, 139, 114, 206, 71], OperandSize::Qword)
}

#[test]
fn vprolq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(XMM27)), operand2: Some(IndirectDisplaced(RDI, 70190600, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(58)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 165, 134, 114, 143, 8, 6, 47, 4, 58], OperandSize::Qword)
}

#[test]
fn vprolq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(XMM11)), operand2: Some(IndirectScaledIndexed(RCX, RSI, Eight, Some(OperandSize::Qword), None)), operand3: Some(Literal8(74)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 165, 156, 114, 12, 241, 74], OperandSize::Qword)
}

#[test]
fn vprolq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(Literal8(45)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 213, 171, 114, 207, 45], OperandSize::Dword)
}

#[test]
fn vprolq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledIndexed(EDX, EDI, Four, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(112)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 205, 172, 114, 12, 186, 112], OperandSize::Dword)
}

#[test]
fn vprolq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexed(EDI, EDX, Four, Some(OperandSize::Qword), None)), operand3: Some(Literal8(89)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 245, 188, 114, 12, 151, 89], OperandSize::Dword)
}

#[test]
fn vprolq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM22)), operand3: Some(Literal8(17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 177, 173, 161, 114, 206, 17], OperandSize::Qword)
}

#[test]
fn vprolq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Four, 1208425884, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 175, 114, 140, 153, 156, 29, 7, 72, 19], OperandSize::Qword)
}

#[test]
fn vprolq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(YMM28)), operand2: Some(IndirectScaledIndexed(RAX, RSI, Two, Some(OperandSize::Qword), None)), operand3: Some(Literal8(41)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 157, 181, 114, 12, 112, 41], OperandSize::Qword)
}

#[test]
fn vprolq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM6)), operand3: Some(Literal8(124)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 245, 207, 114, 206, 124], OperandSize::Dword)
}

#[test]
fn vprolq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledIndexed(ESI, EBX, Four, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 197, 203, 114, 12, 158, 1], OperandSize::Dword)
}

#[test]
fn vprolq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(ZMM1)), operand2: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand3: Some(Literal8(125)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 245, 219, 114, 10, 125], OperandSize::Dword)
}

#[test]
fn vprolq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM4)), operand3: Some(Literal8(87)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 245, 206, 114, 204, 87], OperandSize::Qword)
}

#[test]
fn vprolq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(ZMM27)), operand2: Some(IndirectScaledDisplaced(RCX, Two, 580115645, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(33)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 165, 197, 114, 12, 77, 189, 220, 147, 34, 33], OperandSize::Qword)
}

#[test]
fn vprolq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(ZMM8)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Eight, 1208234792, Some(OperandSize::Qword), None)), operand3: Some(Literal8(95)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 189, 217, 114, 140, 202, 40, 51, 4, 72, 95], OperandSize::Qword)
}

