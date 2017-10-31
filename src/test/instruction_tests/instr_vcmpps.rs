use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcmpps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM3)), operand4: Some(Literal8(106)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 200, 194, 227, 106], OperandSize::Dword)
}

#[test]
fn vcmpps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 314526643, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(14)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 208, 194, 60, 197, 179, 75, 191, 18, 14], OperandSize::Dword)
}

#[test]
fn vcmpps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: Some(Literal8(70)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 232, 194, 208, 70], OperandSize::Qword)
}

#[test]
fn vcmpps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Four, 1615487137, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(21)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 232, 194, 172, 179, 161, 96, 74, 96, 21], OperandSize::Qword)
}

#[test]
fn vcmpps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM4)), operand4: Some(Literal8(71)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 194, 236, 71], OperandSize::Dword)
}

#[test]
fn vcmpps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(109)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 220, 194, 8, 109], OperandSize::Dword)
}

#[test]
fn vcmpps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM2)), operand4: Some(Literal8(36)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 236, 194, 194, 36], OperandSize::Qword)
}

#[test]
fn vcmpps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Four, 1075810456, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(36)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 204, 194, 132, 139, 152, 144, 31, 64, 36], OperandSize::Qword)
}

#[test]
fn vcmpps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM6)), operand4: Some(Literal8(76)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 68, 14, 194, 222, 76], OperandSize::Dword)
}

#[test]
fn vcmpps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(EAX, 801579135, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(7)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 116, 11, 194, 184, 127, 32, 199, 47, 7], OperandSize::Dword)
}

#[test]
fn vcmpps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Four, 1428064976, Some(OperandSize::Dword), None)), operand4: Some(Literal8(8)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 108, 27, 194, 148, 154, 208, 138, 30, 85, 8], OperandSize::Dword)
}

#[test]
fn vcmpps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM18)), operand3: Some(Direct(XMM29)), operand4: Some(Literal8(100)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 145, 108, 1, 194, 229, 100], OperandSize::Qword)
}

#[test]
fn vcmpps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM31)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(42)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 4, 1, 194, 10, 42], OperandSize::Qword)
}

#[test]
fn vcmpps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM25)), operand3: Some(IndirectScaledIndexed(RAX, RSI, Four, Some(OperandSize::Dword), None)), operand4: Some(Literal8(95)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 52, 20, 194, 52, 176, 95], OperandSize::Qword)
}

#[test]
fn vcmpps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM1)), operand4: Some(Literal8(104)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 108, 47, 194, 217, 104], OperandSize::Dword)
}

#[test]
fn vcmpps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(30)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 84, 47, 194, 56, 30], OperandSize::Dword)
}

#[test]
fn vcmpps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(EDI, EDI, Two, Some(OperandSize::Dword), None)), operand4: Some(Literal8(83)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 84, 62, 194, 60, 127, 83], OperandSize::Dword)
}

#[test]
fn vcmpps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM23)), operand3: Some(Direct(YMM27)), operand4: Some(Literal8(101)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 145, 68, 39, 194, 211, 101], OperandSize::Qword)
}

#[test]
fn vcmpps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM28)), operand3: Some(IndirectScaledIndexed(RBX, RDI, Two, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(114)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 28, 35, 194, 44, 123, 114], OperandSize::Qword)
}

#[test]
fn vcmpps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM20)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 2124981645, Some(OperandSize::Dword), None)), operand4: Some(Literal8(125)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 92, 51, 194, 52, 85, 141, 165, 168, 126, 125], OperandSize::Qword)
}

#[test]
fn vcmpps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM2)), operand4: Some(Literal8(80)), lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 68, 27, 194, 202, 80], OperandSize::Dword)
}

#[test]
fn vcmpps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 871114871, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(46)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 100, 74, 194, 12, 197, 119, 40, 236, 51, 46], OperandSize::Dword)
}

#[test]
fn vcmpps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 202613056, Some(OperandSize::Dword), None)), operand4: Some(Literal8(31)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 84, 93, 194, 36, 85, 64, 161, 19, 12, 31], OperandSize::Dword)
}

#[test]
fn vcmpps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM25)), operand3: Some(Direct(ZMM12)), operand4: Some(Literal8(113)), lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 209, 52, 22, 194, 228, 113], OperandSize::Qword)
}

#[test]
fn vcmpps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM16)), operand3: Some(IndirectDisplaced(RCX, 1831430420, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(97)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 124, 70, 194, 153, 20, 105, 41, 109, 97], OperandSize::Qword)
}

#[test]
fn vcmpps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM29)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 140373784, Some(OperandSize::Dword), None)), operand4: Some(Literal8(80)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 20, 84, 194, 28, 253, 24, 239, 93, 8, 80], OperandSize::Qword)
}

