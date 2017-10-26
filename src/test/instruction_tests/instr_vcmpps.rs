use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcmpps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM5)), operand4: Some(Literal8(90)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 200, 194, 197, 90], OperandSize::Dword)
}

#[test]
fn vcmpps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(93)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 192, 194, 19, 93], OperandSize::Dword)
}

#[test]
fn vcmpps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM3)), operand4: Some(Literal8(40)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 224, 194, 203, 40], OperandSize::Qword)
}

#[test]
fn vcmpps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(RBX, RSI, Two, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(75)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 192, 194, 28, 115, 75], OperandSize::Qword)
}

#[test]
fn vcmpps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM5)), operand4: Some(Literal8(117)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 244, 194, 205, 117], OperandSize::Dword)
}

#[test]
fn vcmpps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(110)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 220, 194, 16, 110], OperandSize::Dword)
}

#[test]
fn vcmpps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM2)), operand4: Some(Literal8(110)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 220, 194, 194, 110], OperandSize::Qword)
}

#[test]
fn vcmpps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 419113396, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(42)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 194, 4, 181, 180, 41, 251, 24, 42], OperandSize::Qword)
}

#[test]
fn vcmpps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(10)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 108, 14, 194, 228, 10], OperandSize::Dword)
}

#[test]
fn vcmpps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 252051981, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(34)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 124, 12, 194, 20, 221, 13, 2, 6, 15, 34], OperandSize::Dword)
}

#[test]
fn vcmpps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 606215709, Some(OperandSize::Dword), None)), operand4: Some(Literal8(34)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 100, 29, 194, 44, 117, 29, 30, 34, 36, 34], OperandSize::Dword)
}

#[test]
fn vcmpps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM1)), operand4: Some(Literal8(22)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 68, 9, 194, 217, 22], OperandSize::Qword)
}

#[test]
fn vcmpps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM9)), operand3: Some(IndirectDisplaced(RDI, 296376138, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(112)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 52, 11, 194, 151, 74, 87, 170, 17, 112], OperandSize::Qword)
}

#[test]
fn vcmpps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 71098448, Some(OperandSize::Dword), None)), operand4: Some(Literal8(2)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 92, 28, 194, 28, 157, 80, 224, 60, 4, 2], OperandSize::Qword)
}

#[test]
fn vcmpps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM2)), operand4: Some(Literal8(39)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 108, 45, 194, 202, 39], OperandSize::Dword)
}

#[test]
fn vcmpps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Four, 1115273375, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(58)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 68, 42, 194, 180, 129, 159, 184, 121, 66, 58], OperandSize::Dword)
}

#[test]
fn vcmpps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(ESI, 61894326, Some(OperandSize::Dword), None)), operand4: Some(Literal8(111)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 124, 63, 194, 142, 182, 110, 176, 3, 111], OperandSize::Dword)
}

#[test]
fn vcmpps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM25)), operand3: Some(Direct(YMM21)), operand4: Some(Literal8(85)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 177, 52, 34, 194, 253, 85], OperandSize::Qword)
}

#[test]
fn vcmpps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 1905300237, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(46)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 124, 45, 194, 36, 77, 13, 147, 144, 113, 46], OperandSize::Qword)
}

#[test]
fn vcmpps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM21)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 603509441, Some(OperandSize::Dword), None)), operand4: Some(Literal8(21)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 84, 50, 194, 28, 197, 193, 210, 248, 35, 21], OperandSize::Qword)
}

#[test]
fn vcmpps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM0)), operand4: Some(Literal8(40)), lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 92, 29, 194, 216, 40], OperandSize::Dword)
}

#[test]
fn vcmpps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 379416419, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(86)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 100, 75, 194, 28, 221, 99, 111, 157, 22, 86], OperandSize::Dword)
}

#[test]
fn vcmpps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM0)), operand3: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand4: Some(Literal8(47)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 124, 94, 194, 17, 47], OperandSize::Dword)
}

#[test]
fn vcmpps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM9)), operand3: Some(Direct(ZMM16)), operand4: Some(Literal8(80)), lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 177, 52, 28, 194, 240, 80], OperandSize::Qword)
}

#[test]
fn vcmpps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM26)), operand3: Some(Indirect(RDI, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(105)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 44, 65, 194, 47, 105], OperandSize::Qword)
}

#[test]
fn vcmpps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM28)), operand3: Some(IndirectDisplaced(RDX, 285824272, Some(OperandSize::Dword), None)), operand4: Some(Literal8(6)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 28, 86, 194, 170, 16, 85, 9, 17, 6], OperandSize::Qword)
}

