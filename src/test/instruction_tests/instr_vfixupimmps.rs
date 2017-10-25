use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfixupimmps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: Some(Literal8(27)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 109, 139, 84, 232, 27], OperandSize::Dword)
}

#[test]
fn vfixupimmps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(80)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 117, 143, 84, 32, 80], OperandSize::Dword)
}

#[test]
fn vfixupimmps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Two, 1581663892, Some(OperandSize::Dword), None)), operand4: Some(Literal8(57)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 109, 154, 84, 140, 91, 148, 70, 70, 94, 57], OperandSize::Dword)
}

#[test]
fn vfixupimmps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM11)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(99)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 115, 37, 143, 84, 236, 99], OperandSize::Qword)
}

#[test]
fn vfixupimmps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM27)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 1303010332, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(120)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 99, 37, 134, 84, 12, 117, 28, 92, 170, 77, 120], OperandSize::Qword)
}

#[test]
fn vfixupimmps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Two, 702238070, Some(OperandSize::Dword), None)), operand4: Some(Literal8(127)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 115, 5, 154, 84, 156, 112, 118, 77, 219, 41, 127], OperandSize::Qword)
}

#[test]
fn vfixupimmps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM4)), operand4: Some(Literal8(114)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 85, 169, 84, 228, 114], OperandSize::Dword)
}

#[test]
fn vfixupimmps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(117)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 109, 171, 84, 62, 117], OperandSize::Dword)
}

#[test]
fn vfixupimmps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Eight, 261879419, Some(OperandSize::Dword), None)), operand4: Some(Literal8(93)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 93, 191, 84, 156, 240, 123, 246, 155, 15, 93], OperandSize::Dword)
}

#[test]
fn vfixupimmps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM25)), operand3: Some(Direct(YMM29)), operand4: Some(Literal8(64)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 19, 53, 162, 84, 237, 64], OperandSize::Qword)
}

#[test]
fn vfixupimmps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM11)), operand3: Some(IndirectScaledIndexed(RDX, RDX, Four, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(84)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 37, 171, 84, 36, 146, 84], OperandSize::Qword)
}

#[test]
fn vfixupimmps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM29)), operand3: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand4: Some(Literal8(86)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 227, 21, 179, 84, 51, 86], OperandSize::Qword)
}

#[test]
fn vfixupimmps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM7)), operand4: Some(Literal8(65)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 101, 158, 84, 199, 65], OperandSize::Dword)
}

#[test]
fn vfixupimmps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexed(EDI, ESI, Two, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(118)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 85, 202, 84, 52, 119, 118], OperandSize::Dword)
}

#[test]
fn vfixupimmps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectDisplaced(EDX, 1828903184, Some(OperandSize::Dword), None)), operand4: Some(Literal8(104)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 69, 220, 84, 146, 16, 217, 2, 109, 104], OperandSize::Dword)
}

#[test]
fn vfixupimmps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM25)), operand3: Some(Direct(ZMM5)), operand4: Some(Literal8(43)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 99, 53, 145, 84, 245, 43], OperandSize::Qword)
}

#[test]
fn vfixupimmps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM10)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Two, 1401796249, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(87)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 45, 203, 84, 156, 94, 153, 182, 141, 83, 87], OperandSize::Qword)
}

#[test]
fn vfixupimmps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM14)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 52450641, Some(OperandSize::Dword), None)), operand4: Some(Literal8(12)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 99, 13, 222, 84, 4, 85, 81, 85, 32, 3, 12], OperandSize::Qword)
}

