use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrndscalepd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(88)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 253, 138, 9, 202, 88], OperandSize::Dword)
}

#[test]
fn vrndscalepd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(EDX, EDX, Two, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 253, 143, 9, 36, 82, 18], OperandSize::Dword)
}

#[test]
fn vrndscalepd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Eight, 925780542, Some(OperandSize::Qword), None)), operand3: Some(Literal8(22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 253, 158, 9, 164, 254, 62, 74, 46, 55, 22], OperandSize::Dword)
}

#[test]
fn vrndscalepd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM13)), operand3: Some(Literal8(13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 195, 253, 140, 9, 221, 13], OperandSize::Qword)
}

#[test]
fn vrndscalepd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 1289864924, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(49)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 253, 138, 9, 20, 141, 220, 198, 225, 76, 49], OperandSize::Qword)
}

#[test]
fn vrndscalepd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(XMM22)), operand2: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand3: Some(Literal8(69)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 227, 253, 158, 9, 51, 69], OperandSize::Qword)
}

#[test]
fn vrndscalepd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(Literal8(92)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 253, 172, 9, 216, 92], OperandSize::Dword)
}

#[test]
fn vrndscalepd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(YMM4)), operand2: Some(IndirectDisplaced(EBX, 352207035, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(118)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 253, 170, 9, 163, 187, 64, 254, 20, 118], OperandSize::Dword)
}

#[test]
fn vrndscalepd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Two, 538645195, Some(OperandSize::Qword), None)), operand3: Some(Literal8(89)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 253, 188, 9, 156, 126, 203, 18, 27, 32, 89], OperandSize::Dword)
}

#[test]
fn vrndscalepd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(68)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 227, 253, 169, 9, 205, 68], OperandSize::Qword)
}

#[test]
fn vrndscalepd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(YMM16)), operand2: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 227, 253, 169, 9, 1, 14], OperandSize::Qword)
}

#[test]
fn vrndscalepd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(YMM21)), operand2: Some(IndirectScaledIndexed(RAX, RBX, Eight, Some(OperandSize::Qword), None)), operand3: Some(Literal8(72)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 227, 253, 186, 9, 44, 216, 72], OperandSize::Qword)
}

#[test]
fn vrndscalepd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM5)), operand3: Some(Literal8(40)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 253, 154, 9, 245, 40], OperandSize::Dword)
}

#[test]
fn vrndscalepd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledIndexed(EDX, EAX, Four, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(86)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 253, 205, 9, 12, 130, 86], OperandSize::Dword)
}

#[test]
fn vrndscalepd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectDisplaced(ESI, 1060890416, Some(OperandSize::Qword), None)), operand3: Some(Literal8(38)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 253, 218, 9, 166, 48, 231, 59, 63, 38], OperandSize::Dword)
}

#[test]
fn vrndscalepd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM5)), operand3: Some(Literal8(107)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 227, 253, 153, 9, 229, 107], OperandSize::Qword)
}

#[test]
fn vrndscalepd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(ZMM9)), operand2: Some(Indirect(RCX, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 115, 253, 204, 9, 9, 12], OperandSize::Qword)
}

#[test]
fn vrndscalepd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(ZMM25)), operand2: Some(IndirectDisplaced(RCX, 1300339300, Some(OperandSize::Qword), None)), operand3: Some(Literal8(93)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 99, 253, 217, 9, 137, 100, 154, 129, 77, 93], OperandSize::Qword)
}

