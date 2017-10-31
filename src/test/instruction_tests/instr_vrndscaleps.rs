use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrndscaleps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(66)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 125, 140, 8, 224, 66], OperandSize::Dword)
}

#[test]
fn vrndscaleps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(EDX, EDX, Eight, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 125, 138, 8, 44, 210, 26], OperandSize::Dword)
}

#[test]
fn vrndscaleps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(42)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 125, 157, 8, 32, 42], OperandSize::Dword)
}

#[test]
fn vrndscaleps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(41)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 125, 139, 8, 201, 41], OperandSize::Qword)
}

#[test]
fn vrndscaleps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(XMM26)), operand2: Some(IndirectDisplaced(RDX, 908217400, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(94)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 99, 125, 137, 8, 146, 56, 76, 34, 54, 94], OperandSize::Qword)
}

#[test]
fn vrndscaleps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(RDX, 184074211, Some(OperandSize::Dword), None)), operand3: Some(Literal8(44)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 125, 157, 8, 154, 227, 191, 248, 10, 44], OperandSize::Qword)
}

#[test]
fn vrndscaleps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(Literal8(26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 125, 170, 8, 252, 26], OperandSize::Dword)
}

#[test]
fn vrndscaleps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(YMM0)), operand2: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(82)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 125, 175, 8, 3, 82], OperandSize::Dword)
}

#[test]
fn vrndscaleps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexed(ESI, EDX, Four, Some(OperandSize::Dword), None)), operand3: Some(Literal8(125)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 125, 190, 8, 12, 150, 125], OperandSize::Dword)
}

#[test]
fn vrndscaleps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(56)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 115, 125, 174, 8, 238, 56], OperandSize::Qword)
}

#[test]
fn vrndscaleps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(YMM30)), operand2: Some(IndirectDisplaced(RBX, 1944574255, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 99, 125, 175, 8, 179, 47, 217, 231, 115, 25], OperandSize::Qword)
}

#[test]
fn vrndscaleps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(YMM16)), operand2: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand3: Some(Literal8(98)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 227, 125, 187, 8, 6, 98], OperandSize::Qword)
}

#[test]
fn vrndscaleps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM6)), operand3: Some(Literal8(101)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 125, 154, 8, 206, 101], OperandSize::Dword)
}

#[test]
fn vrndscaleps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectDisplaced(EDI, 1364290172, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 125, 207, 8, 191, 124, 106, 81, 81, 29], OperandSize::Dword)
}

#[test]
fn vrndscaleps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(ZMM0)), operand2: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(80)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 125, 221, 8, 3, 80], OperandSize::Dword)
}

#[test]
fn vrndscaleps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM8)), operand3: Some(Literal8(59)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 83, 125, 153, 8, 200, 59], OperandSize::Qword)
}

#[test]
fn vrndscaleps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Two, 1462483728, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(66)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 125, 202, 8, 180, 89, 16, 187, 43, 87, 66], OperandSize::Qword)
}

#[test]
fn vrndscaleps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(ZMM18)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Four, 312357962, Some(OperandSize::Dword), None)), operand3: Some(Literal8(109)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 227, 125, 217, 8, 148, 153, 74, 52, 158, 18, 109], OperandSize::Qword)
}

