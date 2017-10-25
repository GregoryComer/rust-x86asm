use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcmpps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: Some(Literal8(48)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 232, 194, 205, 48], OperandSize::Dword)
}

#[test]
fn vcmpps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 965444170, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(64)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 194, 12, 205, 74, 130, 139, 57, 64], OperandSize::Dword)
}

#[test]
fn vcmpps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM7)), operand4: Some(Literal8(40)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 208, 194, 247, 40], OperandSize::Qword)
}

#[test]
fn vcmpps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(RDI, RDX, Two, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(72)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 216, 194, 52, 87, 72], OperandSize::Qword)
}

#[test]
fn vcmpps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM2)), operand4: Some(Literal8(70)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 220, 194, 202, 70], OperandSize::Dword)
}

#[test]
fn vcmpps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Eight, 331186098, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(44)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 196, 194, 172, 192, 178, 127, 189, 19, 44], OperandSize::Dword)
}

#[test]
fn vcmpps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM7)), operand4: Some(Literal8(69)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 220, 194, 247, 69], OperandSize::Qword)
}

#[test]
fn vcmpps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(RAX, RDX, Four, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(122)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 196, 194, 28, 144, 122], OperandSize::Qword)
}

#[test]
fn vcmpps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: Some(Literal8(110)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 100, 13, 194, 249, 110], OperandSize::Dword)
}

#[test]
fn vcmpps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 915610693, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(41)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 108, 12, 194, 20, 245, 69, 28, 147, 54, 41], OperandSize::Dword)
}

#[test]
fn vcmpps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(EDX, 1419795919, Some(OperandSize::Dword), None)), operand4: Some(Literal8(20)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 68, 31, 194, 170, 207, 93, 160, 84, 20], OperandSize::Dword)
}

#[test]
fn vcmpps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM14)), operand3: Some(Direct(XMM14)), operand4: Some(Literal8(99)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 209, 12, 10, 194, 230, 99], OperandSize::Qword)
}

#[test]
fn vcmpps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM16)), operand3: Some(IndirectDisplaced(RDI, 877473451, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(68)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 124, 7, 194, 159, 171, 46, 77, 52, 68], OperandSize::Qword)
}

#[test]
fn vcmpps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM13)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 1920734376, Some(OperandSize::Dword), None)), operand4: Some(Literal8(111)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 20, 31, 194, 12, 221, 168, 20, 124, 114, 111], OperandSize::Qword)
}

#[test]
fn vcmpps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM5)), operand4: Some(Literal8(117)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 84, 42, 194, 229, 117], OperandSize::Dword)
}

#[test]
fn vcmpps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(EBX, EBX, Eight, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(64)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 124, 46, 194, 44, 219, 64], OperandSize::Dword)
}

#[test]
fn vcmpps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(ECX, 1819110496, Some(OperandSize::Dword), None)), operand4: Some(Literal8(121)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 116, 58, 194, 153, 96, 108, 109, 108, 121], OperandSize::Dword)
}

#[test]
fn vcmpps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM19)), operand4: Some(Literal8(83)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 177, 76, 41, 194, 235, 83], OperandSize::Qword)
}

#[test]
fn vcmpps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM18)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 29863029, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(3)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 108, 38, 194, 36, 69, 117, 172, 199, 1, 3], OperandSize::Qword)
}

#[test]
fn vcmpps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM28)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Eight, 722155136, Some(OperandSize::Dword), None)), operand4: Some(Literal8(60)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 28, 50, 194, 148, 217, 128, 54, 11, 43, 60], OperandSize::Qword)
}

#[test]
fn vcmpps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM6)), operand4: Some(Literal8(94)), lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 124, 28, 194, 230, 94], OperandSize::Dword)
}

#[test]
fn vcmpps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 840183609, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(52)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 108, 75, 194, 20, 157, 57, 47, 20, 50, 52], OperandSize::Dword)
}

#[test]
fn vcmpps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexed(ECX, EBX, Eight, Some(OperandSize::Dword), None)), operand4: Some(Literal8(25)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 108, 90, 194, 28, 217, 25], OperandSize::Dword)
}

#[test]
fn vcmpps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM20)), operand4: Some(Literal8(57)), lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 177, 116, 30, 194, 236, 57], OperandSize::Qword)
}

#[test]
fn vcmpps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM19)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 1324434885, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(31)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 100, 67, 194, 60, 77, 197, 69, 241, 78, 31], OperandSize::Qword)
}

#[test]
fn vcmpps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM10)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Eight, 1207329580, Some(OperandSize::Dword), None)), operand4: Some(Literal8(65)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 44, 90, 194, 140, 251, 44, 99, 246, 71, 65], OperandSize::Qword)
}

