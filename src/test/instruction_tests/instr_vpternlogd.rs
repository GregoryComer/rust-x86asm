use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpternlogd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: Some(Literal8(65)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 125, 138, 37, 216, 65], OperandSize::Dword)
}

#[test]
fn vpternlogd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(73)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 101, 140, 37, 63, 73], OperandSize::Dword)
}

#[test]
fn vpternlogd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 1192466051, Some(OperandSize::Dword), None)), operand4: Some(Literal8(54)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 69, 156, 37, 52, 213, 131, 150, 19, 71, 54], OperandSize::Dword)
}

#[test]
fn vpternlogd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM17)), operand3: Some(Direct(XMM18)), operand4: Some(Literal8(26)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 179, 117, 135, 37, 234, 26], OperandSize::Qword)
}

#[test]
fn vpternlogd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Eight, 1264381399, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(69)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 99, 77, 140, 37, 164, 254, 215, 237, 92, 75, 69], OperandSize::Qword)
}

#[test]
fn vpternlogd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 1620768433, Some(OperandSize::Dword), None)), operand4: Some(Literal8(89)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 85, 153, 37, 44, 253, 177, 246, 154, 96, 89], OperandSize::Qword)
}

#[test]
fn vpternlogd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM4)), operand4: Some(Literal8(102)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 93, 171, 37, 244, 102], OperandSize::Dword)
}

#[test]
fn vpternlogd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 1282130849, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(67)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 109, 170, 37, 36, 245, 161, 195, 107, 76, 67], OperandSize::Dword)
}

#[test]
fn vpternlogd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Four, 2060074409, Some(OperandSize::Dword), None)), operand4: Some(Literal8(38)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 85, 187, 37, 156, 130, 169, 61, 202, 122, 38], OperandSize::Dword)
}

#[test]
fn vpternlogd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM27)), operand3: Some(Direct(YMM1)), operand4: Some(Literal8(54)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 37, 164, 37, 193, 54], OperandSize::Qword)
}

#[test]
fn vpternlogd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(22)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 115, 85, 170, 37, 46, 22], OperandSize::Qword)
}

#[test]
fn vpternlogd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM30)), operand3: Some(IndirectDisplaced(RDI, 1164861570, Some(OperandSize::Dword), None)), operand4: Some(Literal8(67)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 115, 13, 183, 37, 175, 130, 96, 110, 69, 67], OperandSize::Qword)
}

#[test]
fn vpternlogd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM6)), operand4: Some(Literal8(3)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 101, 201, 37, 206, 3], OperandSize::Dword)
}

#[test]
fn vpternlogd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM7)), operand3: Some(Indirect(ESI, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(113)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 69, 205, 37, 14, 113], OperandSize::Dword)
}

#[test]
fn vpternlogd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM2)), operand3: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand4: Some(Literal8(109)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 109, 221, 37, 25, 109], OperandSize::Dword)
}

#[test]
fn vpternlogd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM24)), operand3: Some(Direct(ZMM12)), operand4: Some(Literal8(107)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 211, 61, 193, 37, 236, 107], OperandSize::Qword)
}

#[test]
fn vpternlogd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM27)), operand3: Some(IndirectDisplaced(RDI, 669251427, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(114)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 227, 37, 195, 37, 183, 99, 247, 227, 39, 114], OperandSize::Qword)
}

#[test]
fn vpternlogd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexed(RSI, RAX, Eight, Some(OperandSize::Dword), None)), operand4: Some(Literal8(23)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 99, 125, 221, 37, 12, 198, 23], OperandSize::Qword)
}

