use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrndscaleps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(36)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 125, 142, 8, 244, 36], OperandSize::Dword)
}

#[test]
fn vrndscaleps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(EDX, Eight, 1636481496, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(37)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 125, 137, 8, 44, 213, 216, 185, 138, 97, 37], OperandSize::Dword)
}

#[test]
fn vrndscaleps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 824391487, Some(OperandSize::Dword), None)), operand3: Some(Literal8(99)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 125, 153, 8, 36, 125, 63, 55, 35, 49, 99], OperandSize::Dword)
}

#[test]
fn vrndscaleps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM8)), operand3: Some(Literal8(44)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 83, 125, 139, 8, 240, 44], OperandSize::Qword)
}

#[test]
fn vrndscaleps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(XMM16)), operand2: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(45)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 227, 125, 140, 8, 0, 45], OperandSize::Qword)
}

#[test]
fn vrndscaleps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(XMM29)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 1777085072, Some(OperandSize::Dword), None)), operand3: Some(Literal8(102)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 99, 125, 158, 8, 44, 213, 144, 42, 236, 105, 102], OperandSize::Qword)
}

#[test]
fn vrndscaleps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(61)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 125, 172, 8, 238, 61], OperandSize::Dword)
}

#[test]
fn vrndscaleps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexed(ESI, EDI, Eight, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 125, 172, 8, 12, 254, 31], OperandSize::Dword)
}

#[test]
fn vrndscaleps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(YMM3)), operand2: Some(IndirectDisplaced(ECX, 1849865564, Some(OperandSize::Dword), None)), operand3: Some(Literal8(15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 125, 188, 8, 153, 92, 181, 66, 110, 15], OperandSize::Dword)
}

#[test]
fn vrndscaleps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM30)), operand3: Some(Literal8(38)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 147, 125, 171, 8, 246, 38], OperandSize::Qword)
}

#[test]
fn vrndscaleps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(YMM15)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 844408843, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(50)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 115, 125, 169, 8, 60, 141, 11, 168, 84, 50, 50], OperandSize::Qword)
}

#[test]
fn vrndscaleps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(YMM4)), operand2: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(85)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 125, 190, 8, 35, 85], OperandSize::Qword)
}

#[test]
fn vrndscaleps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM7)), operand3: Some(Literal8(68)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 125, 158, 8, 239, 68], OperandSize::Dword)
}

#[test]
fn vrndscaleps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Two, 1772919140, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(36)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 125, 203, 8, 132, 120, 100, 153, 172, 105, 36], OperandSize::Dword)
}

#[test]
fn vrndscaleps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 1860405385, Some(OperandSize::Dword), None)), operand3: Some(Literal8(93)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 125, 221, 8, 12, 125, 137, 136, 227, 110, 93], OperandSize::Dword)
}

#[test]
fn vrndscaleps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM13)), operand3: Some(Literal8(3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 83, 125, 158, 8, 205, 3], OperandSize::Qword)
}

#[test]
fn vrndscaleps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(ZMM11)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 1149339283, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(113)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 115, 125, 204, 8, 28, 213, 147, 134, 129, 68, 113], OperandSize::Qword)
}

#[test]
fn vrndscaleps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(ZMM30)), operand2: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand3: Some(Literal8(64)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 99, 125, 223, 8, 55, 64], OperandSize::Qword)
}

