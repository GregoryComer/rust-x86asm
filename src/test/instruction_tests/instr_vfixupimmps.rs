use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfixupimmps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM6)), operand4: Some(Literal8(70)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 77, 140, 84, 238, 70], OperandSize::Dword)
}

#[test]
fn vfixupimmps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(EAX, EBX, Two, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(114)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 117, 143, 84, 36, 88, 114], OperandSize::Dword)
}

#[test]
fn vfixupimmps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(ECX, 1527803843, Some(OperandSize::Dword), None)), operand4: Some(Literal8(4)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 77, 154, 84, 169, 195, 111, 16, 91, 4], OperandSize::Dword)
}

#[test]
fn vfixupimmps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM13)), operand3: Some(Direct(XMM7)), operand4: Some(Literal8(85)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 115, 21, 139, 84, 255, 85], OperandSize::Qword)
}

#[test]
fn vfixupimmps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM25)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Eight, 1260593775, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(43)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 115, 53, 134, 84, 148, 223, 111, 34, 35, 75, 43], OperandSize::Qword)
}

#[test]
fn vfixupimmps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM27)), operand3: Some(IndirectDisplaced(RBX, 1372001837, Some(OperandSize::Dword), None)), operand4: Some(Literal8(93)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 37, 146, 84, 131, 45, 22, 199, 81, 93], OperandSize::Qword)
}

#[test]
fn vfixupimmps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM4)), operand4: Some(Literal8(23)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 69, 171, 84, 212, 23], OperandSize::Dword)
}

#[test]
fn vfixupimmps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(25)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 77, 175, 84, 10, 25], OperandSize::Dword)
}

#[test]
fn vfixupimmps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(EAX, EDX, Eight, Some(OperandSize::Dword), None)), operand4: Some(Literal8(83)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 69, 185, 84, 12, 208, 83], OperandSize::Dword)
}

#[test]
fn vfixupimmps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM13)), operand3: Some(Direct(YMM15)), operand4: Some(Literal8(60)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 67, 21, 170, 84, 207, 60], OperandSize::Qword)
}

#[test]
fn vfixupimmps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(RBX, 1511018892, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(43)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 109, 173, 84, 187, 140, 81, 16, 90, 43], OperandSize::Qword)
}

#[test]
fn vfixupimmps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM23)), operand3: Some(IndirectScaledIndexed(RAX, RBX, Eight, Some(OperandSize::Dword), None)), operand4: Some(Literal8(39)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 115, 69, 181, 84, 20, 216, 39], OperandSize::Qword)
}

#[test]
fn vfixupimmps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM4)), operand4: Some(Literal8(27)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 109, 155, 84, 220, 27], OperandSize::Dword)
}

#[test]
fn vfixupimmps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexed(EDX, EDX, Four, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(60)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 109, 207, 84, 36, 146, 60], OperandSize::Dword)
}

#[test]
fn vfixupimmps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 1889424434, Some(OperandSize::Dword), None)), operand4: Some(Literal8(47)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 69, 223, 84, 28, 205, 50, 84, 158, 112, 47], OperandSize::Dword)
}

#[test]
fn vfixupimmps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM25)), operand4: Some(Literal8(6)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 3, 85, 153, 84, 241, 6], OperandSize::Qword)
}

#[test]
fn vfixupimmps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM8)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 1313860870, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(3)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 115, 61, 203, 84, 52, 197, 6, 237, 79, 78, 3], OperandSize::Qword)
}

#[test]
fn vfixupimmps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM8)), operand3: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand4: Some(Literal8(4)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 227, 61, 222, 84, 26, 4], OperandSize::Qword)
}

