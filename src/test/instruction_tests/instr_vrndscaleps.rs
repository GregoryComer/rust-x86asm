use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrndscaleps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(77)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 125, 143, 8, 254, 77], OperandSize::Dword)
}

#[test]
fn vrndscaleps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(EDX, ECX, Eight, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 125, 139, 8, 44, 202, 15], OperandSize::Dword)
}

#[test]
fn vrndscaleps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 178193930, Some(OperandSize::Dword), None)), operand3: Some(Literal8(56)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 125, 159, 8, 28, 181, 10, 6, 159, 10, 56], OperandSize::Dword)
}

#[test]
fn vrndscaleps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM13)), operand3: Some(Literal8(107)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 83, 125, 143, 8, 197, 107], OperandSize::Qword)
}

#[test]
fn vrndscaleps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(XMM10)), operand2: Some(IndirectScaledIndexed(RSI, RSI, Eight, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(95)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 115, 125, 141, 8, 20, 246, 95], OperandSize::Qword)
}

#[test]
fn vrndscaleps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(XMM13)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Eight, 1726005243, Some(OperandSize::Dword), None)), operand3: Some(Literal8(113)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 115, 125, 158, 8, 172, 249, 251, 191, 224, 102, 113], OperandSize::Qword)
}

#[test]
fn vrndscaleps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Literal8(35)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 125, 171, 8, 195, 35], OperandSize::Dword)
}

#[test]
fn vrndscaleps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexed(EDI, EBX, Two, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 125, 169, 8, 20, 95, 22], OperandSize::Dword)
}

#[test]
fn vrndscaleps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(YMM7)), operand2: Some(IndirectDisplaced(EAX, 1115539618, Some(OperandSize::Dword), None)), operand3: Some(Literal8(37)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 125, 186, 8, 184, 162, 200, 125, 66, 37], OperandSize::Dword)
}

#[test]
fn vrndscaleps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(Literal8(32)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 125, 175, 8, 196, 32], OperandSize::Qword)
}

#[test]
fn vrndscaleps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexed(RSI, RAX, Eight, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 125, 174, 8, 4, 198, 21], OperandSize::Qword)
}

#[test]
fn vrndscaleps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Two, 2095592140, Some(OperandSize::Dword), None)), operand3: Some(Literal8(73)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 125, 191, 8, 132, 121, 204, 50, 232, 124, 73], OperandSize::Qword)
}

#[test]
fn vrndscaleps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM7)), operand3: Some(Literal8(26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 125, 154, 8, 207, 26], OperandSize::Dword)
}

#[test]
fn vrndscaleps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectDisplaced(ECX, 984981986, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 125, 206, 8, 137, 226, 161, 181, 58, 15], OperandSize::Dword)
}

#[test]
fn vrndscaleps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Eight, 1833381703, Some(OperandSize::Dword), None)), operand3: Some(Literal8(16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 125, 220, 8, 180, 211, 71, 47, 71, 109, 16], OperandSize::Dword)
}

#[test]
fn vrndscaleps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM14)), operand3: Some(Literal8(8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 211, 125, 159, 8, 206, 8], OperandSize::Qword)
}

#[test]
fn vrndscaleps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(ZMM11)), operand2: Some(IndirectScaledIndexed(RBX, RSI, Two, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(47)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 115, 125, 202, 8, 28, 115, 47], OperandSize::Qword)
}

#[test]
fn vrndscaleps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(ZMM16)), operand2: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(89)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 227, 125, 220, 8, 0, 89], OperandSize::Qword)
}

