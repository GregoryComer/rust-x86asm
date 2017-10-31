use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcmpps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: Some(Literal8(53)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 208, 194, 216, 53], OperandSize::Dword)
}

#[test]
fn vcmpps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(42)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 240, 194, 11, 42], OperandSize::Dword)
}

#[test]
fn vcmpps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM3)), operand4: Some(Literal8(37)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 208, 194, 243, 37], OperandSize::Qword)
}

#[test]
fn vcmpps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(RDX, RBX, Four, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(7)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 224, 194, 52, 154, 7], OperandSize::Qword)
}

#[test]
fn vcmpps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM0)), operand4: Some(Literal8(44)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 244, 194, 232, 44], OperandSize::Dword)
}

#[test]
fn vcmpps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 971826839, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(118)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 212, 194, 60, 213, 151, 230, 236, 57, 118], OperandSize::Dword)
}

#[test]
fn vcmpps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM5)), operand4: Some(Literal8(78)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 236, 194, 213, 78], OperandSize::Qword)
}

#[test]
fn vcmpps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Four, 1875742492, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(44)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 212, 194, 172, 143, 28, 143, 205, 111, 44], OperandSize::Qword)
}

#[test]
fn vcmpps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM3)), operand4: Some(Literal8(3)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 100, 12, 194, 219, 3], OperandSize::Dword)
}

#[test]
fn vcmpps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(ESI, EAX, Four, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(2)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 124, 12, 194, 52, 134, 2], OperandSize::Dword)
}

#[test]
fn vcmpps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand4: Some(Literal8(70)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 68, 30, 194, 27, 70], OperandSize::Dword)
}

#[test]
fn vcmpps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM23)), operand3: Some(Direct(XMM13)), operand4: Some(Literal8(115)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 209, 68, 3, 194, 253, 115], OperandSize::Qword)
}

#[test]
fn vcmpps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectDisplaced(RBX, 159261352, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(124)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 4, 10, 194, 163, 168, 34, 126, 9, 124], OperandSize::Qword)
}

#[test]
fn vcmpps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Two, 367853788, Some(OperandSize::Dword), None)), operand4: Some(Literal8(44)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 100, 27, 194, 140, 70, 220, 0, 237, 21, 44], OperandSize::Qword)
}

#[test]
fn vcmpps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM1)), operand4: Some(Literal8(69)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 116, 41, 194, 241, 69], OperandSize::Dword)
}

#[test]
fn vcmpps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(EBX, 1091180168, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(61)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 68, 47, 194, 163, 136, 22, 10, 65, 61], OperandSize::Dword)
}

#[test]
fn vcmpps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Eight, 164158435, Some(OperandSize::Dword), None)), operand4: Some(Literal8(114)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 84, 63, 194, 140, 193, 227, 219, 200, 9, 114], OperandSize::Dword)
}

#[test]
fn vcmpps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM18)), operand4: Some(Literal8(64)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 177, 92, 43, 194, 234, 64], OperandSize::Qword)
}

#[test]
fn vcmpps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM10)), operand3: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(52)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 44, 43, 194, 54, 52], OperandSize::Qword)
}

#[test]
fn vcmpps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM8)), operand3: Some(IndirectScaledIndexed(RDX, RSI, Four, Some(OperandSize::Dword), None)), operand4: Some(Literal8(106)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 60, 59, 194, 60, 178, 106], OperandSize::Qword)
}

#[test]
fn vcmpps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM2)), operand4: Some(Literal8(4)), lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 76, 31, 194, 226, 4], OperandSize::Dword)
}

#[test]
fn vcmpps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexed(EAX, EDX, Four, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(9)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 124, 77, 194, 36, 144, 9], OperandSize::Dword)
}

#[test]
fn vcmpps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Two, 1158264044, Some(OperandSize::Dword), None)), operand4: Some(Literal8(20)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 68, 94, 194, 164, 75, 236, 180, 9, 69, 20], OperandSize::Dword)
}

#[test]
fn vcmpps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM4)), operand4: Some(Literal8(126)), lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 68, 27, 194, 220, 126], OperandSize::Qword)
}

#[test]
fn vcmpps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM13)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 1140208348, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(47)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 20, 74, 194, 60, 157, 220, 50, 246, 67, 47], OperandSize::Qword)
}

#[test]
fn vcmpps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM31)), operand3: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand4: Some(Literal8(76)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 4, 85, 194, 23, 76], OperandSize::Qword)
}

