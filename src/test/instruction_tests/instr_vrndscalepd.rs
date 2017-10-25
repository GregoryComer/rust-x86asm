use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrndscalepd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 253, 139, 9, 197, 24], OperandSize::Dword)
}

#[test]
fn vrndscalepd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Eight, 898019128, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(65)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 253, 139, 9, 172, 207, 56, 175, 134, 53, 65], OperandSize::Dword)
}

#[test]
fn vrndscalepd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 1397478483, Some(OperandSize::Qword), None)), operand3: Some(Literal8(53)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 253, 155, 9, 4, 253, 83, 212, 75, 83, 53], OperandSize::Dword)
}

#[test]
fn vrndscalepd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 227, 253, 140, 9, 220, 25], OperandSize::Qword)
}

#[test]
fn vrndscalepd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(XMM28)), operand2: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(72)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 99, 253, 141, 9, 39, 72], OperandSize::Qword)
}

#[test]
fn vrndscalepd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(RDI, Four, 1493192333, Some(OperandSize::Qword), None)), operand3: Some(Literal8(103)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 253, 158, 9, 60, 189, 141, 78, 0, 89, 103], OperandSize::Qword)
}

#[test]
fn vrndscalepd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(74)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 253, 175, 9, 253, 74], OperandSize::Dword)
}

#[test]
fn vrndscalepd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 17971299, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(113)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 253, 175, 9, 60, 117, 99, 56, 18, 1, 113], OperandSize::Dword)
}

#[test]
fn vrndscalepd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 1997299641, Some(OperandSize::Qword), None)), operand3: Some(Literal8(88)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 253, 188, 9, 4, 197, 185, 95, 12, 119, 88], OperandSize::Dword)
}

#[test]
fn vrndscalepd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM0)), operand3: Some(Literal8(45)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 99, 253, 173, 9, 240, 45], OperandSize::Qword)
}

#[test]
fn vrndscalepd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 978275479, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(84)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 253, 175, 9, 28, 117, 151, 76, 79, 58, 84], OperandSize::Qword)
}

#[test]
fn vrndscalepd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(YMM19)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Two, 1248707284, Some(OperandSize::Qword), None)), operand3: Some(Literal8(13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 227, 253, 187, 9, 156, 67, 212, 194, 109, 74, 13], OperandSize::Qword)
}

#[test]
fn vrndscalepd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM3)), operand3: Some(Literal8(10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 253, 153, 9, 211, 10], OperandSize::Dword)
}

#[test]
fn vrndscalepd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 1961006660, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(47)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 253, 204, 9, 60, 149, 68, 150, 226, 116, 47], OperandSize::Dword)
}

#[test]
fn vrndscalepd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 11183453, Some(OperandSize::Qword), None)), operand3: Some(Literal8(87)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 253, 218, 9, 60, 181, 93, 165, 170, 0, 87], OperandSize::Dword)
}

#[test]
fn vrndscalepd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM31)), operand3: Some(Literal8(109)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 147, 253, 158, 9, 247, 109], OperandSize::Qword)
}

#[test]
fn vrndscalepd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(ZMM23)), operand2: Some(IndirectDisplaced(RSI, 1161777457, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 227, 253, 205, 9, 190, 49, 81, 63, 69, 0], OperandSize::Qword)
}

#[test]
fn vrndscalepd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(ZMM11)), operand2: Some(IndirectDisplaced(RAX, 1940867395, Some(OperandSize::Qword), None)), operand3: Some(Literal8(64)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 115, 253, 218, 9, 152, 67, 73, 175, 115, 64], OperandSize::Qword)
}

