use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrndscaleps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(91)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 125, 137, 8, 243, 91], OperandSize::Dword)
}

#[test]
fn vrndscaleps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(EDX, EDX, Two, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 125, 141, 8, 36, 82, 2], OperandSize::Dword)
}

#[test]
fn vrndscaleps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand3: Some(Literal8(24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 125, 153, 8, 62, 24], OperandSize::Dword)
}

#[test]
fn vrndscaleps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM13)), operand3: Some(Literal8(118)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 195, 125, 142, 8, 205, 118], OperandSize::Qword)
}

#[test]
fn vrndscaleps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(XMM25)), operand2: Some(IndirectDisplaced(RBX, 1342402126, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 99, 125, 140, 8, 139, 78, 110, 3, 80, 22], OperandSize::Qword)
}

#[test]
fn vrndscaleps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(XMM27)), operand2: Some(IndirectScaledIndexed(RSI, RDX, Four, Some(OperandSize::Dword), None)), operand3: Some(Literal8(47)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 99, 125, 156, 8, 28, 150, 47], OperandSize::Qword)
}

#[test]
fn vrndscaleps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Literal8(63)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 125, 174, 8, 255, 63], OperandSize::Dword)
}

#[test]
fn vrndscaleps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(YMM4)), operand2: Some(IndirectDisplaced(EDX, 1587644539, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(127)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 125, 175, 8, 162, 123, 136, 161, 94, 127], OperandSize::Dword)
}

#[test]
fn vrndscaleps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(YMM3)), operand2: Some(IndirectDisplaced(EDI, 1673396620, Some(OperandSize::Dword), None)), operand3: Some(Literal8(93)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 125, 188, 8, 159, 140, 1, 190, 99, 93], OperandSize::Dword)
}

#[test]
fn vrndscaleps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM23)), operand3: Some(Literal8(10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 163, 125, 175, 8, 231, 10], OperandSize::Qword)
}

#[test]
fn vrndscaleps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(YMM2)), operand2: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(54)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 125, 173, 8, 16, 54], OperandSize::Qword)
}

#[test]
fn vrndscaleps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(YMM2)), operand2: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand3: Some(Literal8(123)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 125, 188, 8, 23, 123], OperandSize::Qword)
}

#[test]
fn vrndscaleps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM6)), operand3: Some(Literal8(60)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 125, 157, 8, 214, 60], OperandSize::Dword)
}

#[test]
fn vrndscaleps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledIndexed(EDI, ESI, Four, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(119)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 125, 202, 8, 60, 183, 119], OperandSize::Dword)
}

#[test]
fn vrndscaleps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledDisplaced(EDX, Two, 2031074239, Some(OperandSize::Dword), None)), operand3: Some(Literal8(93)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 125, 217, 8, 12, 85, 191, 187, 15, 121, 93], OperandSize::Dword)
}

#[test]
fn vrndscaleps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM5)), operand3: Some(Literal8(44)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 227, 125, 153, 8, 229, 44], OperandSize::Qword)
}

#[test]
fn vrndscaleps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(ZMM20)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 877178779, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(73)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 227, 125, 204, 8, 36, 253, 155, 175, 72, 52, 73], OperandSize::Qword)
}

#[test]
fn vrndscaleps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(ZMM29)), operand2: Some(IndirectDisplaced(RDX, 493360350, Some(OperandSize::Dword), None)), operand3: Some(Literal8(101)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 99, 125, 220, 8, 170, 222, 20, 104, 29, 101], OperandSize::Qword)
}

