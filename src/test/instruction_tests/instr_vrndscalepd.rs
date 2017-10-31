use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrndscalepd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(48)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 253, 141, 9, 194, 48], OperandSize::Dword)
}

#[test]
fn vrndscalepd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(66)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 253, 141, 9, 23, 66], OperandSize::Dword)
}

#[test]
fn vrndscalepd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(ESI, Eight, 1013726863, Some(OperandSize::Qword), None)), operand3: Some(Literal8(54)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 253, 154, 9, 12, 245, 143, 62, 108, 60, 54], OperandSize::Dword)
}

#[test]
fn vrndscalepd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 253, 143, 9, 255, 7], OperandSize::Qword)
}

#[test]
fn vrndscalepd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(XMM19)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Four, 1360638419, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 227, 253, 142, 9, 156, 150, 211, 177, 25, 81, 29], OperandSize::Qword)
}

#[test]
fn vrndscalepd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(XMM19)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 950049011, Some(OperandSize::Qword), None)), operand3: Some(Literal8(87)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 227, 253, 156, 9, 28, 149, 243, 152, 160, 56, 87], OperandSize::Qword)
}

#[test]
fn vrndscalepd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Literal8(121)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 253, 171, 9, 209, 121], OperandSize::Dword)
}

#[test]
fn vrndscalepd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledIndexed(EBX, ESI, Four, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(61)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 253, 169, 9, 52, 179, 61], OperandSize::Dword)
}

#[test]
fn vrndscalepd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexed(ESI, EBX, Two, Some(OperandSize::Qword), None)), operand3: Some(Literal8(96)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 253, 185, 9, 12, 94, 96], OperandSize::Dword)
}

#[test]
fn vrndscalepd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(Literal8(86)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 253, 172, 9, 212, 86], OperandSize::Qword)
}

#[test]
fn vrndscalepd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(YMM25)), operand2: Some(IndirectScaledIndexed(RDX, RAX, Two, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(81)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 99, 253, 170, 9, 12, 66, 81], OperandSize::Qword)
}

#[test]
fn vrndscalepd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(YMM7)), operand2: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand3: Some(Literal8(12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 253, 191, 9, 59, 12], OperandSize::Qword)
}

#[test]
fn vrndscalepd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM3)), operand3: Some(Literal8(93)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 253, 156, 9, 227, 93], OperandSize::Dword)
}

#[test]
fn vrndscalepd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Eight, 1369150009, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(34)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 253, 203, 9, 148, 254, 57, 146, 155, 81, 34], OperandSize::Dword)
}

#[test]
fn vrndscalepd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(ZMM5)), operand2: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand3: Some(Literal8(13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 253, 217, 9, 47, 13], OperandSize::Dword)
}

#[test]
fn vrndscalepd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM28)), operand3: Some(Literal8(48)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 19, 253, 155, 9, 220, 48], OperandSize::Qword)
}

#[test]
fn vrndscalepd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(ZMM11)), operand2: Some(IndirectScaledIndexed(RCX, RDI, Eight, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(95)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 115, 253, 203, 9, 28, 249, 95], OperandSize::Qword)
}

#[test]
fn vrndscalepd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(ZMM25)), operand2: Some(IndirectScaledIndexed(RBX, RAX, Two, Some(OperandSize::Qword), None)), operand3: Some(Literal8(105)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 99, 253, 223, 9, 12, 67, 105], OperandSize::Qword)
}

