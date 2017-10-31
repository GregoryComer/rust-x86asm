use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfixupimmps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM5)), operand4: Some(Literal8(66)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 69, 140, 84, 237, 66], OperandSize::Dword)
}

#[test]
fn vfixupimmps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Four, 1486831471, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(3)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 117, 138, 84, 164, 159, 111, 63, 159, 88, 3], OperandSize::Dword)
}

#[test]
fn vfixupimmps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(ESI, 1363687611, Some(OperandSize::Dword), None)), operand4: Some(Literal8(25)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 101, 154, 84, 166, 187, 56, 72, 81, 25], OperandSize::Dword)
}

#[test]
fn vfixupimmps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM30)), operand3: Some(Direct(XMM11)), operand4: Some(Literal8(4)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 195, 13, 135, 84, 203, 4], OperandSize::Qword)
}

#[test]
fn vfixupimmps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM28)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 332116754, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(84)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 29, 133, 84, 20, 93, 18, 179, 203, 19, 84], OperandSize::Qword)
}

#[test]
fn vfixupimmps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(RDI, RSI, Two, Some(OperandSize::Dword), None)), operand4: Some(Literal8(79)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 117, 154, 84, 28, 119, 79], OperandSize::Qword)
}

#[test]
fn vfixupimmps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM1)), operand4: Some(Literal8(68)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 125, 173, 84, 193, 68], OperandSize::Dword)
}

#[test]
fn vfixupimmps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(ESI, 1993847582, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(71)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 93, 173, 84, 158, 30, 179, 215, 118, 71], OperandSize::Dword)
}

#[test]
fn vfixupimmps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(EDI, ECX, Two, Some(OperandSize::Dword), None)), operand4: Some(Literal8(99)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 69, 190, 84, 12, 79, 99], OperandSize::Dword)
}

#[test]
fn vfixupimmps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM12)), operand4: Some(Literal8(41)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 67, 125, 172, 84, 196, 41], OperandSize::Qword)
}

#[test]
fn vfixupimmps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM13)), operand3: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(45)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 99, 21, 173, 84, 55, 45], OperandSize::Qword)
}

#[test]
fn vfixupimmps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(RDI, RCX, Eight, Some(OperandSize::Dword), None)), operand4: Some(Literal8(64)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 99, 77, 186, 84, 60, 207, 64], OperandSize::Qword)
}

#[test]
fn vfixupimmps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM5)), operand4: Some(Literal8(41)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 77, 153, 84, 221, 41], OperandSize::Dword)
}

#[test]
fn vfixupimmps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Eight, 737356426, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(126)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 69, 205, 84, 164, 193, 138, 42, 243, 43, 126], OperandSize::Dword)
}

#[test]
fn vfixupimmps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 64457717, Some(OperandSize::Dword), None)), operand4: Some(Literal8(109)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 69, 220, 84, 60, 213, 245, 139, 215, 3, 109], OperandSize::Dword)
}

#[test]
fn vfixupimmps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM24)), operand3: Some(Direct(ZMM16)), operand4: Some(Literal8(60)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 179, 61, 148, 84, 200, 60], OperandSize::Qword)
}

#[test]
fn vfixupimmps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM30)), operand3: Some(IndirectScaledIndexed(RDX, RDX, Two, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(47)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 115, 13, 193, 84, 12, 82, 47], OperandSize::Qword)
}

#[test]
fn vfixupimmps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM18)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Eight, 121454173, Some(OperandSize::Dword), None)), operand4: Some(Literal8(97)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 227, 109, 210, 84, 180, 241, 93, 62, 61, 7, 97], OperandSize::Qword)
}

